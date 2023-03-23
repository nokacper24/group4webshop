import { useEffect, useState } from "react";
import { AccordionBody } from "./AccordionBody";
import { AccordionHeader, AccordionHeaderProps } from "./AccordionHeader";
import { AccordionRowProps } from "./AccordionRow";
import { ChangeType } from "./ChangeTypes";

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

export function AccordionSection(props: AccordionSectionProps) {
  const deleteRow = (id: number) => {
    content.rows = content.rows.filter((row) => row.id !== id);
    setContent({ ...content });
    props.registerContentChange(id, ChangeType.Delete);
  };

  const editRow = (id: number) => {
    console.log("edit: " + id);
    props.registerContentChange(id, ChangeType.Edit);
  };

  const addRow = (title: string) => {
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

  const deleteSelf = () => {
    console.log("Delete self");
    console.log(props);
    props.deleteSection(props.sectionID);
  }

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
