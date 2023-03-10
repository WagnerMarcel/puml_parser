
#ifndef _STRUCTS_H_
#define _STRUCTS_H_

struct A
{
    int a, b, c, d;
};

struct B
{
    long a, b, c, d;
};

struct C
{
    float a, b, c, d;
};

class D
{
    double a, b, c, d;
};

#endif

class E
{
public:
    void get() {}

    unsigned int m_myInt;

protected:
    int m_protectedInt = -33;

private:
    int someFunc() {}

    float m_someVar{0.42F};
};

class F
{
    double a, b, c, d;
};
