import { useEffect, useState } from "react";
import { AccordionBody } from "./AccordionBody";
import { AccordionHeader, AccordionHeaderProps } from "./AccordionHeader";
import { AccordionRowProps } from "./AccordionRow";
import { ChangeType } from "./ChangeTypes";

/**
 * The props of the AccordionSection component.
 */
export type AccordionSectionProps = {
  header: {
    title: string;
  };
  rows: {
    title: string;
    id: number;
  }[];

  sectionID: number;
  registerContentChange: (id: number, change: ChangeType) => void;
  deleteSection: (id: number) => void;
};

let latestID = 100;
/**
 * The main component for managing a header and its body.
 *
 * @param props the props of the component, must be of AccordionSectionProps type
 * @returns the React component for the Accordion section
 */
export function AccordionSection(props: AccordionSectionProps) {
  /**
   * Calls the deleteSection function in the parent component. Deleting itself in the process.
   */
  const deleteSelf = () => {
    console.log("Delete self");
    console.log(props);
    props.deleteSection(props.sectionID);
  };

  /**
   * React recommends that when two children need the same data, it is best to move it to the parent.
   * The row control system has therefore been moved from AccordionBody.
   */

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

  return (
    <>
      <AccordionHeader
        title={props.header.title}
        deleteSelf={deleteSelf}
        addRow={addRow}
      ></AccordionHeader>
      <AccordionBody
        rows={rows}
        editRow={editRow}
        deleteRow={deleteRow}
      ></AccordionBody>
    </>
  );
}
