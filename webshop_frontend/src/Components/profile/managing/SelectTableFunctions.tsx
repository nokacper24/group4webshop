import { SelectTableRowProps } from "./SelectTable";

interface Button {
  text: string;
  action: (indices: number[]) => void;
}

export function createSelectTableProps(
  headers: string[],
  rows: SelectTableRowProps[],
  buttonText: string,
  buttonAction: (index: number) => void,
  outsideButtons: Map<string, (selectedIndex: number[]) => void>
) {
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

export function createTableRowProps(
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
