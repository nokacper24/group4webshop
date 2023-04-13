import React, { useState, useEffect } from "react";
import SelectTableHeader from "./SelectTableHeader";
import SelectTableRow from "./SelectTableRow";
import SelectTableOutsideButton from "./SelectTableOutsideButton";
import { Button, OutsideButton } from "../../../Interfaces";

export type SelectTableProps = {
  header: {
    columns: {
      text: string;
    }[];
  };
  rows: {
    id: string;
    columns: {
      text: string;
    }[];
  }[];
  button: Button;
  outsideButtons: OutsideButton[];
};

export type SelectTableRowProps = {
  id: string;
  columns: { text: string }[];
};

/**
 * Represents a Select Table component.
 * A table where multiple rows can be selected for an action.
 * The table starts with a "select" column, and ends with a column
 * of buttons that perform an action of the row it is in.
 *
 * @param props Table header and rows, and button after table.
 * @returns A Select Table component.
 */
export default function SelectTable(props: SelectTableProps) {
  const [selectedRows, setSelectedRows] = useState(0);
  const [selectedRowsIndices, setSelectedRowsIndices] = useState<number[]>([]);
  const [selectAll, setSelectAll] = useState("none");

  const clearSelected = () => {
    setSelectedRows(0);
    setSelectedRowsIndices([]);
  };

  const selectAllIndices = () => {
    let allIndices = [];
    for (let i = 0; i < props.rows.length; i++) {
      allIndices.push(i);
    }
    setSelectedRowsIndices([...allIndices]);
  };

  const selectNoIndices = () => {
    setSelectedRowsIndices([]);
  };

  const toggleSelectAll = () => {
    if (selectedRows != props.rows.length) {
      setSelectedRows(props.rows.length);
      selectAllIndices();
    } else {
      setSelectedRows(0);
      selectNoIndices();
    }
  };

  const incrementSelectedRows = () => {
    setSelectedRows(selectedRows + 1);
  };
  const decrementSelectedRows = () => {
    setSelectedRows(selectedRows - 1);
  };

  const updateSelected = (checked: boolean, index: number) => {
    setSelectAll("some");
    if (checked) {
      incrementSelectedRows();
      setSelectedRowsIndices([...selectedRowsIndices, index]);
    } else {
      decrementSelectedRows();
      let newSelected = selectedRowsIndices.filter((item) => item != index);
      setSelectedRowsIndices(newSelected);
    }
  };

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
        <table className="select-table">
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
                key={row.id}
                rowIndex={index}
                columns={row.columns}
                updateSelected={updateSelected}
                selectAll={selectAll}
                button={props.button}
              />
            ))}
          </tbody>
        </table>
      </div>
      <div className="button-container">
        {props.outsideButtons.map((button) => {
          return (
            <SelectTableOutsideButton
              key={button.text}
              text={button.text}
              indices={selectedRowsIndices}
              action={button.action}
              clearSelected={clearSelected}
            />
          );
        })}
      </div>
    </React.Fragment>
  );
}
