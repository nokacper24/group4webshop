import { useEffect, useState } from "react";
import { AccordionHeader } from "./AccordionHeader";
import { AccordionRowProps } from "./AccordionRow";

export default function AccordionTable() {
  const editAccordion = (id: number) => {
    console.log("edit: " + id);
  };

  const deleteAccordion = (id: number) => {
    console.log("delete: " + id);
    setRows((rows) => rows.filter((row) => row.id !== id));
  };

  const rows: AccordionRowProps[] = [
    {
      title: "Test",
      id: 1,
      editFunction: editAccordion,
      removeFunction: deleteAccordion,
    },
    {
      title: "Test2",
      id: 2,
      editFunction: editAccordion,
      removeFunction: deleteAccordion,
    },
  ];

  const [rowList, setRows] = useState<AccordionRowProps[]>(rows);

  return (
    <div className="accordion-table">
      <AccordionHeader rows={rowList}></AccordionHeader>
    </div>
  );
}
