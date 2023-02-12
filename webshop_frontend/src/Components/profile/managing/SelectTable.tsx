import React from "react";
import SelectTableRow, { SelectTableRowProps } from "./SelectTableRow";

/**
 * The cell type can be either checkbox, string, button, or danger-button.
 */
export type SelectTableProps = {
  header: {
    columns: {
      text: string;
    }[];
  };
  rows: SelectTableRowProps[];
  outsideButton: { text: string; type: string };
};

export default function SelectTable(props: SelectTableProps) {
  let outsideButton;
  switch (props.outsideButton.type) {
    case "button":
      outsideButton = (
        <button className="default-button small-button">
          {props.outsideButton.text}
        </button>
      );
      break;
    case "danger-button":
      outsideButton = (
        <button className="default-button danger small-button">
          {props.outsideButton.text}
        </button>
      );
      break;
    default:
      break;
  }

  return (
    <React.Fragment>
      <div className="table-container">
        <table className="select-table table-container">
          <thead>
            <tr>
              {props.header.columns.map((column, index) => (
                <th key={index}>{column.text}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {props.rows.map((row, index) => (
              <SelectTableRow key={index} columns={row.columns} />
            ))}
          </tbody>
        </table>
      </div>
      {outsideButton}
    </React.Fragment>
  );
}
