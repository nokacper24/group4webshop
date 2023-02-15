import React from "react";
import { useParams } from "react-router-dom";
import SelectTable from "./SelectTable";

/**
 * A Manage License Access page.
 * The license manager can choose which users in their company can
 * access the license.
 *
 * @returns A Manage License Access page component.
 */
export default function ManageLicenseAccess() {
  const { id } = useParams();

  const withoutAccessTable = {
    header: {
      columns: [{ text: "Users" }, { text: "Access" }],
    },
    rows: [
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Add", type: "button" },
        ],
      },
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Add", type: "button" },
        ],
      },
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Add", type: "button" },
        ],
      },
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Add", type: "button" },
        ],
      },
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Add", type: "button" },
        ],
      },
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Add", type: "button" },
        ],
      },
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Add", type: "button" },
        ],
      },
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Add", type: "button" },
        ],
      },
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Add", type: "button" },
        ],
      },
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Add", type: "button" },
        ],
      },
    ],
    actionButton: { text: "Add all selected", type: "button" },
  };

  const withAccessTable = {
    header: {
      columns: [{ text: "Users" }, { text: "Access" }],
    },
    rows: [
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Remove", type: "danger-button" },
        ],
      },
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Remove", type: "danger-button" },
        ],
      },
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Remove", type: "danger-button" },
        ],
      },
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Remove", type: "danger-button" },
        ],
      },
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Remove", type: "danger-button" },
        ],
      },
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Remove", type: "danger-button" },
        ],
      },
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Remove", type: "danger-button" },
        ],
      },
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Remove", type: "danger-button" },
        ],
      },
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Remove", type: "danger-button" },
        ],
      },
      {
        columns: [
          { text: "user@company.com", type: "string" },
          { text: "Remove", type: "danger-button" },
        ],
      },
    ],
    actionButton: { text: "Remove all selected", type: "danger-button" },
  };

  return (
    <React.Fragment>
      <section className="container left-aligned">
        <h1>Manage license access</h1>
        <p>
          {/* TODO: Fetch product name of license */}
          License {id}
        </p>
      </section>

      <section className="container left-aligned">
        <h2>Users without access</h2>

        <SelectTable
          header={withoutAccessTable.header}
          rows={withoutAccessTable.rows}
          actionButton={withoutAccessTable.actionButton}
        />
      </section>

      <section className="container left-aligned">
        <h2>Users with access</h2>
        <SelectTable
          header={withAccessTable.header}
          rows={withAccessTable.rows}
          actionButton={withAccessTable.actionButton}
        />
      </section>
    </React.Fragment>
  );
}
