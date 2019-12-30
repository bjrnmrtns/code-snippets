clang-query test.cpp -- -std=c++11
match cxxMethodDecl(isPublic(), ofClass(hasName("bmystruct")))

