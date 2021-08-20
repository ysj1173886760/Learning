#include <unordered_map>
#include <stdexcept>
#include <string>
#include <cassert>

template <typename Resource, typename Identifier>
class ResourceHolder {
public:
    void load(Identifier id, const std::string &filename) {
        std::unique_ptr<Resource> resource(new Resource());
        if (!resource->loadFromFile(filename))
            throw std::runtime_error("ResourceHolder::load: Fail to load " + filename);
        
        insertResource(id, std::move(resource));
    }

    template <typename Parameter>
    void load(Identifier id, const std::string &filename, const Parameter &secondParam) {
        std::unique_ptr<Resource> resource(new Resource());
        if (!resource->loadFromFile(filename, secondParam))
            throw std::runtime_error("ResourceHolder::load: Fail to load " + filename);
        
        insertResource(id, std::move(resource));
    }

    Resource *get(Identifier id) {
        return const_cast<Resource *>((static_cast<const ResourceHolder<Resource, Identifier> *>(this))->get(id));
    }

    const Resource *get(Identifier id) const {
        auto result = _resourceMap.find(id);
        assert(result != _resourceMap.end());

        return result->second.get();
    }

private:
    void insertResource(Identifier id, std::unique_ptr<Resource> resource) {
        auto result = _resourceMap.insert(std::make_pair(id, std::move(resource)));
        assert(result.second);
    }

private:
    std::unordered_map<Identifier, std::unique_ptr<Resource>> _resourceMap;
};