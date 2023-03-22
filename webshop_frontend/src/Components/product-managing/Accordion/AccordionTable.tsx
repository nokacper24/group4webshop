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
    setContentChanges((changes) => {
      for (let type in ChangeType) {
        changes.set(ChangeType[type as keyof typeof ChangeType], []);
      }
      return changes;
    });
  });

  const registerContentChange = (id: number, change: ChangeType) => {
    if (!contentChanges.get(change)?.includes(id)) {
      contentChanges.get(change)?.push(id);
    }
    if (change === ChangeType.Delete) {
      contentChanges
        .get(ChangeType.Edit)
        ?.filter((changeId) => changeId !== id);
      priorityChanges.delete(id);
    }
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
