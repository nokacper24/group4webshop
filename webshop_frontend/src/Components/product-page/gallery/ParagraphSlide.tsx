import { SlidesProps } from "./Gallery";

export type ParagraphSlideProps = {
  paragraph: String,
  reviewerProfile: {
    picturePath: string;
    name: string;
    title: string;
  };
}

export const ParagraphSlide = (props: ParagraphSlideProps) => {
  return <div className="slide">
    <p>
      {
        props.paragraph
      }
    </p>
    <div className="reviewer">
      <img src={props.reviewerProfile.picturePath} alt="" />
      <p>{props.reviewerProfile.name}</p>
      <p>{props.reviewerProfile.title}</p>
    </div>
  </div>
};
