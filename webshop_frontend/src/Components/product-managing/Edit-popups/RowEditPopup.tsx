/**
 * Props for the RowEditPopup component.
 * Image indicates if the content should be interpreted as an image or not.
 * The title and content can be undefined if a new row is being created.
 * If the content is an image, the title is the alt text.
 */
export type RowEditPopupProps = {
  image: boolean;
  title: string | undefined;
  content: string | undefined;
};

export default function RowEditPopup(props: RowEditPopupProps) {
  return (
    <div className="popup-grey-zone">
      <div className="popup-box">
        <form className="popup-form">
          {props.image ? (
            <>
            </>
          ) : (
            <>
                rows={10}
              />
            </>
          )}

        </form>
      </div>
    </div>
  );
}
