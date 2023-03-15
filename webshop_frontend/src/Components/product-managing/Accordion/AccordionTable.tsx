import { useEffect, useState } from "react";
import { AccordionHeader, AccordionHeaderProps } from "./AccordionHeader";
import { AccordionRowProps } from "./AccordionRow";
import { AccordionSection } from "./AccordionSection";
import { ChangeType } from "./ChangeTypes";

export default function AccordionTable() {
  const [changes, setChanges] = useState<Map<string, number[]>>(new Map());

  useEffect(() => {
    // Sets up the map so that it can register changes
    setChanges((changes) => {
      for (let type in ChangeType) {
        changes.set(type, []);
      }
      return changes;
    });
  });



  const registerChange = (id: number, change: ChangeType) => {
    if (!changes.get(change)?.includes(id)) {
      changes.get(change)?.push(id);
    }
    if (change === ChangeType.DELETE) {
      changes.get(ChangeType.EDIT)?.filter((changeId) => changeId !== id);
      changes.get(ChangeType.MOVE)?.filter((changeId) => changeId !== id);
    }
  };

  const [headerList, setHeaders] = useState<AccordionHeaderProps[]>([]);

  const addHeader = (title: string) => {
    const id = headerList.length;
    setHeaders((headers) => [
      ...headers,
      {
        title: title,
        id: id,
        addRow: addRow,
        rows: rowList,
      },
    ]);
  };

  const [sections, setSections] = useState<[]>();

  const rows = [
    {
      title: "Test",
      id: 1,
    },
    {
      title: "Test2",
      id: 2,
    },
  ];

  return (
    <div className="accordion-table">
      <AccordionSection
        rows={rows}
        registerChange={registerChange}
      ></AccordionSection>
    </div>
  );
}
