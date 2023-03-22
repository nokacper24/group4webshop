import { useEffect, useState } from "react";
import { AccordionHeader, AccordionHeaderProps } from "./AccordionHeader";
import { AccordionRowProps } from "./AccordionRow";
import { AccordionSection } from "./AccordionSection";
import { ChangeType } from "./ChangeTypes";

export default function AccordionTable() {
  const [priorityChanges, setPriorityChanges] = useState<Map<number, number>>(
    new Map()
  ); //ID as key, priority as value
  const [contentChanges, setContentChanges] = useState<
    Map<ChangeType, number[]>
  >(new Map()); //The type of change as key, list of IDs that has had that change as value

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

  const [sectionList, setSectionList] =
    useState<AccordionSectionProps[]>(sections);
  return (
    <div className="accordion-table">
      <AccordionSection
        rows={rows}
        registerChange={registerChange}
      ></AccordionSection>
    </div>
  );
}
