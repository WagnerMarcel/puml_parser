struct TestInterface
{
    int m_someMember;
};

class TestClass
{
public:
    enum class TestEnum
    {
        One,
        Two
    };
    TestEnum m_enumValue;

    void get(TestInterface &testEnum) {}

    unsigned int m_myInt;

protected:
    struct TestStruct
    {
        int one{1};
    };
    TestStruct m_structValue;

    int m_protectedInt = -33;

private:
    int someFunc(TestInterface &testStruct) {}

    float m_someVar{0.42F};
};
