typedef unsigned short half;

#ifdef __cplusplus
extern "C" {
#endif
// 补齐函数声明，对应gemm/gemm_SRAM.mlu
void gemm16Kernel(half *outputDDR, int8_t *input1DDR, int8_t *input2DDR,
	uint32_t m, uint32_t k, uint32_t n, int16_t pos);
#ifdef __cplusplus
}
#endif
