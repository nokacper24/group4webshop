import { useEffect, useState } from "react";
import SelectTableCell, { SelectTableCellProps } from "./SelectTableCell";

export type SelectTableRowProps = {
  columns: SelectTableCellProps[];
  updateSelected: (checked: boolean) => void;
  selectAll: string;
};

/**
 * Represents a single row in the Select Table component.
 *
 * @param props The columns in the row.
 * @returns The table row component.
 */
export default function SelectTableRow(props: SelectTableRowProps) {
  const [selected, setSelected] = useState(false);

  useEffect(() => {
    if (props.selectAll == "all") {
      setSelected(true);
    } else if (props.selectAll == "none") {
      setSelected(false);
    }
  });

  const toggleSelect = () => {
    props.updateSelected(!selected);
    setSelected((selected) => !selected);
  };

  return (
    <tr className={`${selected ? "selected-row" : ""}`}>
      <td>
        <input
          type="checkbox"
          checked={selected}
          onChange={() => toggleSelect()}
        />
      </td>
      {props.columns.map((column, index) => (
        <SelectTableCell key={index} text={column.text} type={column.type} />
      ))}
    </tr>
  );
}
