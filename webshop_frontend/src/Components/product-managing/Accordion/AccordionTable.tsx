import { useEffect, useState } from "react";
import { AccordionHeader } from "./AccordionHeader";
import { AccordionRowProps } from "./AccordionRow";

export default function AccordionTable() {
    const [rowList, setRows] = useState<AccordionRowProps[]>();

  const editAccordion = (id: number) => {
    console.log("edit");
  };

  const deleteAccordion = (id: number) => {
    console.log("delete");
  };

  return (
    <div className="accordion-table">
      <AccordionHeader rows={rows}></AccordionHeader>
    </div>
  );
}
