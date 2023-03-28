import { SelectTableProps, SelectTableRowProps } from "./SelectTable";

interface Button {
  text: string;
  action: (indices: number[]) => void;
}

/**
 * Create an object that represents the props for a Select Table.
 *
 * @param headers The table headers.
 * @param rows The rows in the table body.
 * @param buttonText The text in the button for each row.
 * @param buttonAction The function that is called when the button in each row is clicked.
 * @param outsideButtons The buttons outside the table itself, affecting selected rows in the table.
 * @returns An object that represents the props for a Select Table component.
 */
export function createSelectTableProps(
  headers: string[],
  rows: SelectTableRowProps[],
  buttonText: string,
  buttonAction: (index: number) => void,
  outsideButtons: Map<string, (selectedIndex: number[]) => void>
): SelectTableProps {
  let headerColumns = headers.map((text) => {
    return {
      text: text,
    };
  });

  let outsideButtonItems: Button[] = [];
  outsideButtons.forEach((value, key) => {
    outsideButtonItems.push({
      text: key,
      action: value,
    });
  });

  return {
    header: {
      columns: headerColumns,
    },
    rows: rows,
    button: { text: buttonText, action: buttonAction },
    outsideButtons: outsideButtonItems,
  };
}

/**
 * Create an object that represents the props for a row in a Select Table.
 *
 * @param id The row's ID.
 * @param text The text in the row. Each item in the list represents a column in the row.
 * @returns An object that represents the props for a row.
 */
export function createRowProps(
  id: string,
  text: string[]
): SelectTableRowProps {
  return {
    id: id,
    columns: text.map((text) => {
      return {
        text: text,
      };
    }),
  };
}

/**
 * Move an item from one Select Table to another.
 *
 * @param index The index of the item in the rows.
 * @param fromTable The table the item is moved from.
 * @param toTable The table the item is moved to.
 * @param setFromListFunction The set function that sets the 'from list'.
 * @param setToListFunction The set function that sets the 'to list'.
 * @returns The item that was moved.
 */
export function moveItemBetweenTables(
  index: number,
  fromTable: SelectTableProps,
  toTable: SelectTableProps,
  setFromListFunction: any,
  setToListFunction: any
) {
  /* Get the item to be moved from the "From list" to the "To list" */
  let item: SelectTableRowProps = fromTable.rows[index];

  /* Remove item from the "From list" */
  let newFromTempArray = fromTable.rows.filter((element) => element !== item);
  setFromListFunction(newFromTempArray);

  /* Add item to the "To list" */
  toTable.rows.push(item);
  setToListFunction(toTable.rows);

  return item;
}

/**
 * Update the lists keeping track of changes, when an item is moved from one table to another.
 * Does not store every change made, only if it differs from the original data.
 *
 * @param item The item that has been moved.
 * @param newFromList The list the item is moved from.
 * @param newToList The list the item is moved to.
 */
export function updateNewChanges(
  item: SelectTableRowProps,
  newFromList: Set<string>,
  newToList: Set<string>
) {
  if (newFromList.has(item.id)) {
    newFromList.delete(item.id);
  } else if (!newToList.has(item.id)) {
    newToList.add(item.id);
  }
}
