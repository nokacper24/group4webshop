import { useState } from "react";
import SelectTableCell, { SelectTableCellProps } from "./SelectTableCell";

export type SelectTableRowProps = {
  columns: SelectTableCellProps[];
};

/**
 * Represents a single row in the Select Table component.
 *
 * @param props The columns in the row.
 * @returns The table row component.
 */
export default function SelectTableRow(props: SelectTableRowProps) {
  const [selected, setSelected] = useState(false);

  const toggleSelect = () => {
    setSelected((selected) => !selected);
  };

  return (
    <tr className={`${selected ? "selected-row" : ""}`}>
      <td>
        <input type="checkbox" onClick={toggleSelect} />
      </td>
      {props.columns.map((column, index) => (
        <SelectTableCell key={index} text={column.text} type={column.type} />
      ))}
    </tr>
  );
}
