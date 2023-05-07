import { useState } from "react";
import { AccordionBody } from "./AccordionBody";
import { AccordionHeader } from "./AccordionHeader";
import { ChangeType } from "./ChangeTypes";
import { showPopup } from "../Edit-popups/RowEditPopup";
import { SimpleDescription } from "../../../../../Interfaces";

/**
 * The props of the AccordionSection component.
 */
export type AccordionSectionProps = {
  header: {
    title: string;
  };
  rows: SimpleDescription[];

  sectionID: number;
};

type PrivateAccordionSectionProps = {
  header: {
    title: string;
  };
  rows: SimpleDescription[];

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
export function AccordionSection(props: PrivateAccordionSectionProps) {
  const [collapse, setCollapse] = useState<boolean>(false);

  /**
   * Calls the deleteSection function in the parent component. Deleting itself in the process.
   */
  const deleteSelf = () => {
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
  const addRow = () => {
    // Temporary debug solution
    if (rows.length < 2) {
      showPopup({
        image: false,
        title: undefined,
        content: undefined,
        informationCallBack: finishCreation,
      });
    }
    function finishCreation(image: boolean, title: string, content: string) {
      let id = createID();
      props.registerContentChange(id, ChangeType.Add);

      rows.push({
        component_id: id,
        text: image ? undefined : { text_title: title, paragraph: content },
        image: image ? { image_path: content, alt_text: title } : undefined,
        is_text_not_image: !image,
      });
      setRows([...rows]);
    }
  };

  const createID = (): number => {
    latestID = latestID + Math.floor(Math.random() * 13);
    return props.sectionID + latestID;
  };

  /**
   * Deletes a row from the body of the section.
   *
   * @param id the ID of the row to be deleted
   */
  const deleteRow = (id: number) => {
    let newRows = rows.filter((row) => row.component_id !== id);
    setRows(newRows);
    props.registerContentChange(id, ChangeType.Delete);
  };

  /**
   * Initializes the process of changing the content of a row.
   *
   * @param id the ID of the row to be edited
   */
  const editRow = (id: number) => {
    let row = rows.find((row) => row.component_id === id);
    if (row) {
      showPopup({
        image: !row.is_text_not_image,
        title: row.is_text_not_image
          ? row.text?.text_title
          : row.image?.alt_text,
        content: row.is_text_not_image
          ? row.text?.paragraph
          : row.image?.image_path,
        informationCallBack: finishEdit,
      });
      function finishEdit(image: boolean, title: string, content: string) {
        //Tried creating a if (row) but sonarlint complains row always exists, but the TS/React compiler complains it might not exist
        row!.is_text_not_image = !image;
        row!.text = image
          ? undefined
          : { text_title: title, paragraph: content };
        row!.image = image
          ? { image_path: content, alt_text: title }
          : undefined;
        setRows([...rows]);
        props.registerContentChange(id, ChangeType.Edit);
      }
    }
    props.registerContentChange(id, ChangeType.Edit);
  };

  const [rows, setRows] = useState<SimpleDescription[]>([...props.rows]);

  const swapRows = () => {
    let newRows = [...rows];
    newRows.reverse();
    setRows(newRows);
  };

  const collapseBody = () => {
    setCollapse(!collapse);
  };

  return (
    <>
      <AccordionHeader
        title={props.header.title}
        collapseFunction={collapseBody}
        deleteSelf={deleteSelf}
        addRow={addRow}
      ></AccordionHeader>
      <AccordionBody
        rows={rows}
        collapsed={collapse}
        swapRows={swapRows}
        editRow={editRow}
        deleteRow={deleteRow}
      ></AccordionBody>
    </>
  );
}
