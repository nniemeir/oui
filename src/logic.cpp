#include "../include/oui.h"

// Search records for OUI that matches the one in MAC variable. Return
// corresponding name if match found, return nothing otherwise.
std::optional<record> searchRecords(std::ifstream &csvFile,
                                    std::string search_term) {
  record targetRecord;
  bool foundRecord = false;
  std::string manufacturerIDField;
  std::string manufacturerNameField;
  // Search the first field of each line in the CSV file until a match is found
  // or EOF
  while (std::getline(csvFile, manufacturerIDField, ',') && !foundRecord) {
    std::getline(csvFile, manufacturerNameField, '\n');
    if (manufacturerIDField == search_term) {
      // Assign the contents of each field in the line to the corresponding
      // variable in targetRecord and exit the loop
      targetRecord.manufacturerID = manufacturerIDField;
      // Remove extra commas at end of second field
      targetRecord.manufacturerName = manufacturerNameField.substr(
          0, manufacturerNameField.size() - SUFFIX_LENGTH);
      return targetRecord;
    }
  }
  return std::nullopt;
}

bool validateInput(const std::string &mac) {
  if (mac.length() > MIN_MAC_LENGTH && mac.length() < MAX_MAC_LENGTH) {
    return true;
  } else {
    return false;
  }
}

// Alter input string to match formatting of the CSV file
std::string formatMAC(const std::string &mac) {
  std::string searchTerm = mac;
  // Remove punctuation symbols and spaces from target string
  searchTerm.erase(remove_if(searchTerm.begin(), searchTerm.end(), ispunct),
                   searchTerm.end());
  searchTerm.erase(remove_if(searchTerm.begin(), searchTerm.end(), isspace),
                   searchTerm.end());

  // Truncate the target MAC address to 6 characters, the length of the
  // manufacturer ID
  searchTerm.resize(MANUFACTURER_ID_LENGTH);
  // Capitalize all letters in target string so it will be in the same style as
  // the CSV
  transform(searchTerm.begin(), searchTerm.end(), searchTerm.begin(), toupper);
  return searchTerm;
}