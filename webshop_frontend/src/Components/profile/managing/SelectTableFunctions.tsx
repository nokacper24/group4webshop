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
