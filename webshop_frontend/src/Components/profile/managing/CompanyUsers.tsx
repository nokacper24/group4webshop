import { Link, useParams } from "react-router-dom";

export default function CompanyUsers() {
  const { companyId } = useParams();

  return (
    <>
      <section className="container left-aligned">
        <h1>Manage users</h1>
      </section>
    </>
  );
}
