/*
 * Copyright (c) 2020 One of A Kind Technologies B.V.
 */
namespace {
class MessageHandler
{
public:
    template <typename TMessageType>
    void Register(std::function<void(TMessageType & message)> callback)
    {
        mapping.insert(std::make_pair(TMessageType::id, [&callback](std::vector<char> & messageData) {
            TMessageType message = TMessageType::Deserialize(messageData); // call deserialize
            callback(message);
        }));
    }
    template <typename TClassType, typename TMessageType>
    void RegisterMember(TClassType & instance, void (TClassType::*function)(TMessageType &))
    {
        auto memberFunction = std::mem_fn(function);
        Register<TMessageType>([&instance, &memberFunction](TMessageType & message) {
            memberFunction(instance, message);
        });
    }
    void Handle(std::vector<char> & data)
    {
    }

private:
    std::map<int, std::function<void(std::vector<char> &)>> mapping;
};
} // namespace

class TestClass
{
public:
    void CallbackFunction(Vic::Cammgr::Messages::Version &)
    {
    }
    void SomeFunction()
    {
        MessageHandler registry;
        registry.RegisterMember<TestClass, Vic::Cammgr::Messages::Version>(*this, &TestClass::CallbackFunction);
    }
};