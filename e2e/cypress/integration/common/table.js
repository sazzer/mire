/**
 * Helper to convert a Cucumber DataTable that consists of a single list of values into just that list
 * @param dataTable The data table
 */
export function toList(dataTable) {
  return dataTable.rawTable.map((row) => row[0]);
}

/**
 * Helper to convert a Cucumber DataTable where the first column is field names and the other columns are values into
 * a list of objects, where each object has names from the first column and appropriate values from the subsequent columns.
 * @param dataTable The data table
 */
export function toObjectsTall(dataTable) {
  // The outer list from dataTable.rawTable has one element per row.
  // Each element in this is then an array with one element per cell in that row.

  const rawData = dataTable.rawTable;
  const result = [];
  const numberOfObjects = rawData[0].length;

  for (let i = 1; i < numberOfObjects; ++i) {
    const value = {};
    rawData.forEach((row) => {
      value[row[0].toLowerCase()] = row[i];
    });
    result.push(value);
  }

  cy.log(result);
  return result;
}
