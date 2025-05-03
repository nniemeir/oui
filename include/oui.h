#ifndef OUI_H_
#define OUI_H_

#include <algorithm>
#include <fstream>
#include <iostream>
#include <optional>
#include <string>

#define MANUFACTURER_ID_LENGTH 6
#define MIN_MAC_LENGTH 12
#define MAX_MAC_LENGTH 17
#define SUFFIX_LENGTH 2
#define FILE_NAME "/usr/share/oui/IEEE_OUI.csv"

struct record {
  std::string manufacturerID;
  std::string manufacturerName;
};

bool validateInput(const std::string &mac);
std::optional<record> searchRecords(std::ifstream &csvFile,
                                    std::string search_term);
std::string formatMAC(const std::string &mac);

#endif