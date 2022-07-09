#include <iostream>
#include <longbridge.hpp>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longbridge;

int main(int argc, char const *argv[]) {
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  Config config;
  Status status = Config::from_env(config);
  QuoteContext::create(config,
                       [](auto res) { std::cout << res.is_ok() << std::endl; });
  std::cin.get();
}
