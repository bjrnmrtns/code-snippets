struct A {
    int x;
};

int tobecalled(int a)
{
    return a;
}

int func()
{
    A a { 3 };
    return tobecalled(3);
}
