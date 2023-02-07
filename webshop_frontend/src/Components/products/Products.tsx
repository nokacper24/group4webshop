import { ProductCard, ProductCardProps } from "./ProductCard";

export type State = {
  products: ProductCardProps[];
};

export default function Products() {
  const website_data = [
    {
      name: "Product1",
      description:
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
      sourceImage: "https://unsplash.it/300/200",
    },
    {
      name: "Product2",
      description:
        "Lorem ipsum dolor sit amet consectetur adipisicing elit. Fugit laboriosam ea ipsam, non hic obcaecati repellat dignissimos deleniti explicabo nemo, aliquam, doloremque ipsa! Reiciendis, quidem ea asperiores impedit vero quisquam!",
      sourceImage: "https://unsplash.it/300/200",
    },
    {
      name: "Product3",
      description:
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
      sourceImage: "https://unsplash.it/300/200",
    },
    {
      name: "Product4",
      description:
        "Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus dignissimos placeat dicta. Deserunt, velit, rerum distinctio iusto, officiis nulla omnis culpa obcaecati illo quaerat eius asperiores? Voluptates sunt nemo magnam.",
      sourceImage: "https://unsplash.it/300/200",
    },
  ];
  let state: State = {
    products: [],
  };
  website_data.forEach((projectObject) => {
    let newProduct: ProductCardProps = {
      props: projectObject,
    };
    state.products.push(newProduct);
  });

  return (
    <div>
      <section className="container">
        <h1>Our solutions</h1>
        <ul className="product-list grid-container">
          {state.products.map((product) => (
            <ProductCard key={product.props.name} props={product.props} />
          ))}
        </ul>
      </section>
    </div>
  );
}
