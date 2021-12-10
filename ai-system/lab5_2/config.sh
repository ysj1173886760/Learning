#!/bin/bash

current_wd=$(pwd)
# set env variable
cd /opt/code_chap_5_student/env
source env.sh

# copy to pluginops
dir1=/opt/code_chap_5_student/code_chap_5_1_student/src/bangc/PluginPowerDifferenceOp
dir2=/opt/code_chap_5_student/env/Cambricon-CNPlugin-MLU270/pluginops

so_file=/opt/code_chap_5_student/env/Cambricon-CNPlugin-MLU270/build/libcnplugin.so
neuware_pth=/opt/code_chap_5_student/env/neuware/

buildPlugin() {
    cp -r $dir1 $dir2 
    if [ $? -ne 0 ]
    then 
        echo "failed to copy plugin ops"
        cd $current_wd
        return 1
    fi
    echo "building plugin"

    # build cnplugin
    cd /opt/code_chap_5_student/env/Cambricon-CNPlugin-MLU270
    ./build_cnplugin.sh
}

if [ ! -d "$dir2/PluginPowerDifferenceOp" ]
then
    # if we don't have the folder, then we copy it directly
    buildPlugin
else
    # if we have the folder, then we check the difference
    diff $dir1 "$dir2/PluginPowerDifferenceOp" >/dev/null
    if [ $? -ne 0 -o ! -f "$so_file" ]
    then
        buildPlugin
    else
        echo "skip building plugin"
    fi
fi
# set -e will terminate the console if you are using source config.sh to execute this script

# copy to env/neuware
cp $so_file "$neuware_pth/lib64/"

if [ $? -ne 0 ]
then 
    echo "failed to copy libcnplugin.so"
    cd $current_wd
    return 1
fi

cp "$dir1/cnplugin.h" "$neuware_pth/include"

if [ $? -ne 0 ]
then
    echo "failed to copy cnplugin.h"
    cd $current_wd
    return 1
fi

# copy to tensorflow
dir3=/opt/code_chap_5_student/code_chap_5_1_student/src/tf-implementation/tf-add-power-diff
dir4=/opt/code_chap_5_student/env/tensorflow-v1.10/tensorflow

cp $dir3/cwise_op_power_difference* $dir4/core/kernels/

if [ $? -ne 0 ]
then 
    echo "failed to copy power_difference"
    cd $current_wd
    return 1
fi

cp $dir3/BUILD $dir4/core/kernels/
if [ $? -ne 0 ]
then 
    echo "failed to copy BUILD"
    cd $current_wd
    return 1
fi

cp $dir3/mlu_stream.h $dir4/stream_executor/mlu/

if [ $? -ne 0 ]
then 
    echo "failed to copy mlu_stream.h"
    cd $current_wd
    return 1
fi

cp $dir3/mlu_lib_ops.* $dir4/stream_executor/mlu/mlu_api/lib_ops/

if [ $? -ne 0 ]
then 
    echo "failed to copy mlu_lib_ops"
    cd $current_wd
    return 1
fi

cp $dir3/mlu_ops.h $dir4/stream_executor/mlu/mlu_api/ops/

if [ $? -ne 0 ]
then 
    echo "failed to copy mlu_ops.h"
    cd $current_wd
    return 1
fi

cp $dir3/power_difference.cc $dir4/stream_executor/mlu/mlu_api/ops/

if [ $? -ne 0 ]
then 
    echo "failed to copy power_difference.cc"
    cd $current_wd
    return 1
fi

cp $dir3/math_ops.cc $dir4/core/ops/

if [ $? -ne 0 ]
then 
    echo "failed to copy math_ops.cc"
    cd $current_wd
    return 1
fi

# build tensorflow
dir_tf=/opt/code_chap_5_student/env/tensorflow-v1.10
cd $dir_tf
./build_tensorflow-v1.10_mlu.sh