
namespace OtherNamespace
{
    class OtherParent
    {
        float m_test{};

        struct NestedInParent
        {
            float m_pub{};
        };

        NestedInParent m_nest{};
    };
}

namespace TestNamespace
{
    class ParentClass
    {
    private:
        int m_priv{};

    protected:
        float m_prot{};

    public:
        unsigned int m_pub{};
    };

    class TestClass : public ParentClass, public OtherNamespace::OtherParent
    {
    public:
        int m_pub{};
    };
}
