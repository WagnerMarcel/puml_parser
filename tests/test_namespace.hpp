namespace TestNamespace
{
    class TestClass
    {
    public:
        enum class TestEnum
        {
            One,
            Two
        };
        TestEnum m_enumValue;

        void get() {}

        unsigned int m_myInt;

    protected:
        struct TestStruct
        {
            int one{1};
        };
        TestStruct m_structValue;

        int m_protectedInt = -33;

    private:
        int someFunc() {}

        float m_someVar{0.42F};
    };

} // namespace TestNamespace
