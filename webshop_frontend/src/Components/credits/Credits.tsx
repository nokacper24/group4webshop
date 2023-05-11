/**
 * Represents the Credits page.
 *
 * @returns The Credits  component as a JSX element.
 */
export default function Credits() {
  return (
    <>
      <section className="container left-aligned">
        <h2>Credits</h2>
        <p>
          Images used are either generated with Dall-E 2, Midjourney or are
          public domain. Links to public domain images can be found in the list
          below.
        </p>
        <ul>
          <li>
            <a href="">Some img</a>
          </li>
          <li>
            <a href="">Some img</a>
          </li>
          <li>
            <a href="">Some img</a>
          </li>
        </ul>
      </section>
    </>
  );
}
