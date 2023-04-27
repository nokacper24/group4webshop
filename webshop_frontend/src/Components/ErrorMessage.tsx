interface ErrorMessageProps {
  message: string;
}
export function ErrorMessage(props: ErrorMessageProps) {
  const { message } = props;
  return (
    <>
      <section className="container">
        <h1>Something went wrong!</h1>
        <p>{message}</p>
      </section>
    </>
  );
}
