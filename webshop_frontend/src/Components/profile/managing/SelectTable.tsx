import React, { useState, useEffect } from "react";
import SelectTableHeader from "./SelectTableHeader";
import SelectTableRow from "./SelectTableRow";

/**
 * The cell type can be either string, button, or danger-button.
 */
export type SelectTableProps = {
  header: {
    columns: {
      text: string;
    }[];
  };
  rows: {
    columns: {
      text: string;
      type: string;
    }[];
  }[];
  actionButton: { text: string; type: string };
};

/**
 * Represents a Select Table component.
 * A table where multiple rows can be selected for an action.
 *
 * @param props Table header and rows, and button after table.
 * @returns A Select Table component.
 */
export default function SelectTable(props: SelectTableProps) {
  const [selectedRows, setSelectedRows] = useState(0);
  const [selectAll, setSelectAll] = useState("none");

  const toggleSelectAll = () => {
    if (selectedRows != props.rows.length) {
      if (selectAll == "all") {
        setSelectedRows(0);
      } else {
        setSelectedRows(props.rows.length);
      }
    } else {
      if (selectAll == "none") {
        setSelectedRows(props.rows.length);
      } else {
        setSelectedRows(0);
      }
    }
  };

  const incrementSelectedRows = () => {
    setSelectedRows(selectedRows + 1);
  };
  const decrementSelectedRows = () => {
    setSelectedRows(selectedRows - 1);
  };

  const updateSelected = (checked: boolean) => {
    setSelectAll("some");
    if (checked) {
      incrementSelectedRows();
    } else {
      decrementSelectedRows();
    }
  };

  let actionButton;
  switch (props.actionButton.type) {
    case "button":
      actionButton = (
        <button className="default-button small-button">
          {props.actionButton.text}
        </button>
      );
      break;
    case "danger-button":
      actionButton = (
        <button className="default-button danger small-button">
          {props.actionButton.text}
        </button>
      );
      break;
    default:
      break;
  }

  useEffect(() => {
    if (selectedRows == props.rows.length) {
      setSelectAll("all");
    } else if (selectedRows == 0) {
      setSelectAll("none");
    } else {
      setSelectAll("some");
    }
  });

  return (
    <React.Fragment>
      <div className="table-container">
        <table className="select-table table-container">
          <thead>
            <SelectTableHeader
              columns={props.header.columns}
              toggleSelectAll={() => toggleSelectAll()}
              selectAll={selectAll}
            />
          </thead>
          <tbody>
            {props.rows.map((row, index) => (
              <SelectTableRow
                key={index}
                columns={row.columns}
                updateSelected={updateSelected}
                selectAll={selectAll}
              />
            ))}
          </tbody>
        </table>
      </div>
      <div className="button-container">{actionButton}</div>
    </React.Fragment>
  );
}
