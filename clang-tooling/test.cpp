struct bmystruct {
    int bmembervar;
    void bdomember() {};
};

int bfreefunction(int bparam)
{
    return bparam;
}

int func()
{
    bmystruct blocalvar { 3 };
    blocalvar.bdomember();
    return bfreefunction(3);
}
