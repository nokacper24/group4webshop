import { useEffect, useState } from "react";
import SelectTableButton from "./SelectTableButton";
import { Button } from "../../../Interfaces";

/**
 * id: the row's ID.
 * rowIndex: the row's index in the list, starts at 0.
 * columns: a list of the text cells in the table row.
 * updateSelected: inform parent the checked status of this row.
 * selected: if this row is selected.
 * button: the action button to be put in the last column of the row.
 */
type RowProps = {
  id: string;
  rowIndex: number;
  columns: {
    text: string;
  }[];
  updateSelected: (checked: boolean, id: string) => void;
  selected: boolean;
  button: Button;
};

/**
 * Represents a single row in the Select Table component.
 *
 * @param props The columns in the row.
 * @returns The table row component.
 */
export default function SelectTableRow(props: RowProps) {
  const toggleSelect = () => {
    props.updateSelected(!props.selected, props.id);
  };

  return (
    <tr className={props.selected ? "selected-row" : ""}>
      <td>
        <label className="hidden-label">Select</label>
        <input
          type="checkbox"
          checked={props.selected}
          onChange={toggleSelect}
        />
      </td>
      {props.columns.map((column, index) => (
        <td key={index}>{column.text}</td>
      ))}
      <td>
        <SelectTableButton
          id={props.id}
          text={props.button.text}
          action={props.button.action}
        />
      </td>
    </tr>
  );
}
