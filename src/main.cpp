/*
OUI Lookup - Given a MAC address, match the OUI to a manufacturer's name using
IEEE records

Author: Jacob Niemeir <nniemeir@protonmail.com>
*/

#include "../include/oui.h"

int main(int argc, char *argv[]) {
  if (argc != 2) {
    std::cout << "oui only takes a single argument.\n";
    return EXIT_FAILURE;
  }

  std::string MAC = argv[1];
  std::ifstream csvFile;
  csvFile.open(FILE_NAME);
  if (!csvFile.is_open()) {
    std::cout << "Unable to access " << FILE_NAME << "\n";
    return EXIT_FAILURE;
  }

  if (MAC.length() >= MIN_MAC_LENGTH && MAC.length() <= MAX_MAC_LENGTH) {
    std::string searchTerm = formatMAC(MAC);
    // Our manufacturer ID table and the target string are passed to the
    // recordSearch function
    std::optional<record> result = searchRecords(csvFile, searchTerm);
    csvFile.close();
    if (result.has_value()) {
      std::cout << result.value().manufacturerName << "\n";
    } else {
      std::cout << "Manufacturer not found\n";
    }
  } else {
    std::cout << "Invalid MAC Address\n";
    return EXIT_FAILURE;
  }
  return EXIT_SUCCESS;
}
