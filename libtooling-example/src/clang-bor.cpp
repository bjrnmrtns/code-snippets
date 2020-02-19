#include "clang/AST/ASTConsumer.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/Frontend/CompilerInstance.h"
#include "clang/Frontend/FrontendAction.h"
#include "clang/Tooling/Tooling.h"
#include "clang/Tooling/CommonOptionsParser.h"
#include "clang/AST/ASTContext.h"
#include <string>

static llvm::cl::OptionCategory functionNamesCategory("definition-to-measurement options");
static llvm::cl::extrahelp CommonHelp(clang::tooling::CommonOptionsParser::HelpMessage);
static llvm::cl::extrahelp MoreHelp("\nMore help text...\n");

class DeclVisitor
    : public clang::RecursiveASTVisitor<DeclVisitor>
{
public:
    explicit DeclVisitor(const std::string & functionName) :
        functionName(functionName)
    {
    }

    bool VisitDeclRefExpr(clang::DeclRefExpr * declRefExpr)
    {
        std::string declName(declRefExpr->getDecl()->getName());
        if (declName.find("projection") != std::string::npos)
        {
            llvm::outs() << declName << "\n";
        }
        return true;
    }
    bool VisitBreakStmt(clang::BreakStmt * breakStatement)
    {
        llvm::outs() << "break;" << "\n";
        return true;
    }
    const std::string functionName;
};

class BorVisitor : public clang::RecursiveASTVisitor<BorVisitor>
{
public:
    explicit BorVisitor(clang::ASTContext *)
    {
    }
    bool VisitFunctionDecl(clang::FunctionDecl * fd)
    {
        std::string functionName = fd->getNameInfo().getName().getAsString();
        DeclVisitor declVisitor(functionName);
        declVisitor.TraverseStmt(fd->getBody());
        return true;
    }
};

class BorConsumer : public clang::ASTConsumer
{
public:
    explicit BorConsumer(clang::ASTContext * context) : visitor(context) {}

    virtual void HandleTranslationUnit(clang::ASTContext & context)
    {
        visitor.TraverseDecl(context.getTranslationUnitDecl());
    }

private:
    BorVisitor visitor;
};

class BorAction : public clang::ASTFrontendAction
{
public:
    virtual std::unique_ptr<clang::ASTConsumer> CreateASTConsumer(
        clang::CompilerInstance & Compiler, llvm::StringRef InFile)
    {
        return std::unique_ptr<clang::ASTConsumer>(
            new BorConsumer(&Compiler.getASTContext()));
    }
};

int main(int argc, const char ** argv)
{
    clang::tooling::CommonOptionsParser optionsParser(argc, argv, functionNamesCategory);
    clang::tooling::ClangTool tool(optionsParser.getCompilations(), optionsParser.getSourcePathList());
    return tool.run(clang::tooling::newFrontendActionFactory<BorAction>().get());
}
