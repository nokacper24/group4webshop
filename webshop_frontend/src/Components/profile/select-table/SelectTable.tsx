import { useState, useEffect } from "react";
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
  rows: SelectTableRowProps[];
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
  const [selectedRows, setSelectedRows] = useState<Map<string, boolean>>(
    new Map()
  );
  const [allSelected, setAllSelected] = useState<boolean>(false);

  const getSelectedRows = () => {
    let rows: string[] = [];

    selectedRows.forEach((value, key) => {
      if (value) {
        rows.push(key);
      }
    });

    return rows;
  };

  /**
   * Set all rows to be either selected or not.
   *
   * @param value If the rows are to be set as selected or not.
   */
  const setAllSelectedRows = (value: boolean) => {
    let tempMap = new Map<string, boolean>();

    selectedRows.forEach((_, key) => {
      tempMap.set(key, value);
    });

    setSelectedRows(tempMap);
  };

  const selectAllIndices = () => {
    setAllSelectedRows(true);
  };

  const selectNoIndices = () => {
    setAllSelectedRows(false);
  };

  const toggleSelectAll = () => {
    setAllSelected((oldAllSelected) => {
      if (oldAllSelected) {
        selectNoIndices();
      } else {
        selectAllIndices();
      }
      return !oldAllSelected;
    });
  };

  const checkIfAllSelected = (tempMap: Map<string, boolean>) => {
    let value = true;

    tempMap.forEach((v) => {
      if (!v) {
        value = false;
      }
    });

    return value;
  };

  const updateSelected = (checked: boolean, id: string) => {
    let tempMap = new Map(selectedRows);
    tempMap.set(id, checked);
    setSelectedRows(tempMap);

    setAllSelected(checkIfAllSelected(tempMap));
  };

  useEffect(() => {
    // Add each ID of props.rows to the selectedRows map with default false
    let tempMap = new Map<string, boolean>();

    props.rows.forEach((row) => {
      tempMap.set(row.id, false);
    });

    setSelectedRows(tempMap);

    console.log("Prop rows: ", props.rows);
    console.log("State rows: ", tempMap);
    console.log(" ");
  }, [props.rows]);

  return (
    <>
      <div className="table-container">
        <table className="select-table">
          <thead>
            <SelectTableHeader
              columns={props.header.columns}
              toggleSelectAll={toggleSelectAll}
              selectAll={allSelected}
            />
          </thead>
          <tbody>
            {props.rows.map((row, index) => (
              <SelectTableRow
                key={row.id}
                id={row.id}
                rowIndex={index}
                columns={row.columns}
                updateSelected={updateSelected}
                selected={selectedRows.get(row.id)!}
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
              selectedIds={getSelectedRows()}
              action={button.action}
              clearSelected={selectNoIndices}
            />
          );
        })}
      </div>
    </>
  );
}
