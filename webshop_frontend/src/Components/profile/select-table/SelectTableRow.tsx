import { useEffect, useState } from "react";
import SelectTableButton from "./SelectTableButton";
import { Button } from "./SelectTable";

/**
 * rowIndex: the row's index in the list, starts at 0.
 * columns: a list of the text cells in the table row.
 * updateSelected: inform parent the checked status of this row.
 * selectAll: information about the  status of the "select all" checkbox.
 * button: the action button to be put in the last column of the row.
 */
export type RowProps = {
  rowIndex: number;
  columns: {
    text: string;
  }[];
  updateSelected: (checked: boolean, index: number) => void;
  selectAll: string;
  button: Button;
};

/**
 * Represents a single row in the Select Table component.
 *
 * @param props The columns in the row.
 * @returns The table row component.
 */
export default function SelectTableRow(props: RowProps) {
  const [selected, setSelected] = useState(false);

  useEffect(() => {
    if (props.selectAll == "all") {
      setSelected(true);
    } else if (props.selectAll == "none") {
      setSelected(false);
    }
  });

  const toggleSelect = () => {
    props.updateSelected(!selected, props.rowIndex);
    setSelected((selected) => !selected);
  };

  return (
    <tr className={`${selected ? "selected-row" : ""}`}>
      <td>
        <label className="hidden-label">Select</label>
        <input type="checkbox" checked={selected} onChange={toggleSelect} />
      </td>
      {props.columns.map((column, index) => (
        <td key={index}>{column.text}</td>
      ))}
      <td>
        <SelectTableButton
          key={props.button.text}
          rowIndex={props.rowIndex}
          text={props.button.text}
          action={props.button.action}
        />
      </td>
    </tr>
  );
}
