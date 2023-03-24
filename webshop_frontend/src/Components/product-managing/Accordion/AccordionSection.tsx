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
    rows: {
      title: string;
      id: number;
    }[];
  };

  sectionID: number;
  registerContentChange: (id: number, change: ChangeType) => void;
  deleteSection: (id: number) => void;
};

/**
 * The main component for managing a header and its body.
 *
 * @param props the props of the component, must be of AccordionSectionProps type
 * @returns the React component for the Accordion section
 */
export function AccordionSection(props: AccordionSectionProps) {
  /**
   * Deletes a row from the body of the section.
   *
   * @param id the ID of the row to be deleted
   */
  const deleteRow = (id: number) => {
    content.rows = content.rows.filter((row) => row.id !== id);
    setContent({ ...content });
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

  /**
   * Initializes the process of adding a row to the body of the section.
   *
   * @param title title of the row to be added
   */
  const addRow = (title: string) => {
    //Temporary debug solution
    if (content.rows.length < 2) {
      content.rows.push({
        title: title,
        id: createID(),
        editFunction: editRow,
        removeFunction: deleteRow,
      });
      setContent({ ...content });
    }
  };

  var latestID = 100;
  const createID = (): number => {
    return latestID++;
  };

  /**
   * Calls the deleteSection function in the parent component. Deleting itself in the process.
   */
  const deleteSelf = () => {
    console.log("Delete self");
    console.log(props);
    props.deleteSection(props.sectionID);
  };

  const [content, setContent] = useState<AccordionHeaderProps>({
    title: props.header.title,
    rows: props.header.rows.map((row) => {
      return {
        title: row.title,
        id: row.id,
        editFunction: editRow,
        removeFunction: deleteRow,
      };
    }),
    addRow: addRow,
    deleteSelf: deleteSelf,
  });

  return (
    <>
      <AccordionHeader
        title={content.title}
        rows={content.rows}
        addRow={content.addRow}
        deleteSelf={content.deleteSelf}
      ></AccordionHeader>
      <AccordionBody rows={content.rows}></AccordionBody>
    </>
  );
}
