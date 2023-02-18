import { GalleryProps } from "./Gallery";

export type ParagraphSlideProps = {
  id: string;
  paragraph: string;
  reviewerProfile: {
    picturePath: string;
    name: string;
    title: string;
  };
};

export const ParagraphSlide = (props: ParagraphSlideProps) => {
  return (
    <div className="slide" id={props.id}>
      <p>{props.paragraph}</p>
      <div className="reviewer-profile">
        <img
          src={props.reviewerProfile.picturePath}
          alt=""
          className="reviewer-picture"
        />
        <div className="reviewer-details">
          <p>{props.reviewerProfile.name}</p>
          <p>{props.reviewerProfile.title}</p>
        </div>
      </div>
    </div>
  );
};
