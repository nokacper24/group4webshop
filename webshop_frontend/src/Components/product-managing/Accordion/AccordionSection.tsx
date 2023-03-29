import { useEffect, useState } from "react";
import { AccordionBody, addNewRow } from "./AccordionBody";
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

  return (
    <>
      <AccordionHeader
        title={props.header.title}
        addRow={addNewRow}
        deleteSelf={deleteSelf}
      ></AccordionHeader>
      <AccordionBody rows={props.rows} registerContentChange={props.registerContentChange}></AccordionBody>
    </>
  );
}
