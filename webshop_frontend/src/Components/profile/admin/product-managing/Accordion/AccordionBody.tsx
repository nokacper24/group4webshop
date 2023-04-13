import { useEffect, useState } from "react";
import { AccordionRow, AccordionRowProps } from "./AccordionRow";
import { ChangeType } from "./ChangeTypes";

export type AccordionBodyProps = {
  rows: {
    title: string;
    id: number;
  }[];
  registerContentChange: (id: number, change: ChangeType) => void;
};

let addRowFunc: (title: string) => void;
var latestID = 100;

/**
 * Simple component that keeps the rows of the accordion body.
 *
 * @param props the props of the component, must be of AccordionBodyProps type
 * @returns the React component for the Accordion body
 */
export function AccordionBody(props: AccordionBodyProps) {
  /**
   * Initializes the process of adding a row to the body of the section.
   *
   * @param title title of the row to be added
   */
  const addRow = (title: string) => {
    //Temporary debug solution
    if (rows.length < 2) {
      rows.push({
        title: title,
        id: createID(),
      });
      setRows([...rows]);
    }
  };

  const createID = (): number => {
    return latestID++;
  };

  /**
   * Deletes a row from the body of the section.
   *
   * @param id the ID of the row to be deleted
   */
  const deleteRow = (id: number) => {
    let newRows = rows.filter((row) => row.id !== id);
    setRows(newRows);
    props.registerContentChange(id, ChangeType.Delete);
  };

  /**
   * Initializes the process of changing the content of a row.
   *
   * @param id the ID of the row to be edited
   */
  const editRow = (id: number) => {
    console.log("edit: " + id);
    props.registerContentChange(id, ChangeType.Edit);
  };

  const [rows, setRows] = useState<{ title: string; id: number }[]>([
    ...props.rows,
  ]);

  useEffect(() => {
    addRowFunc = addRow; //Set so that we can call it from outside the component
  });

  return (
    <div className="accordion-body">
      {rows.map((row) => {
        return (
          <AccordionRow
            key={"row" + row.id}
            title={row.title}
            id={row.id}
            editFunction={editRow}
            removeFunction={deleteRow}
          ></AccordionRow>
        );
      })}
    </div>
  );
}

export function addNewRow(title: string) {
  addRowFunc(title);
}
