import React, { useState, useEffect } from "react";
import SelectTableHeader from "./SelectTableHeader";
import SelectTableRow from "./SelectTableRow";
import SelectTableButton from "./SelectTableButton";

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
  button: {
    text: string;
    action: (index: number) => void;
  };
  outsideButtons: { text: string /* action: () => void  */ }[];
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
        {props.outsideButtons.map((button, index) => {
          return (
            <SelectTableButton
              key={button.text}
              rowIndex={-1}
              text={button.text}
              action={props.button.action}
            />
          );
        })}
      </div>
    </React.Fragment>
  );
}
