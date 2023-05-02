import "./App.css";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Header from "./Components/Header";
import Footer from "./Components/Footer";
import Home from "./Components/home/Home";
import Products from "./Components/products/Products";
import ProductPage from "./Components/product-page/ProductPage";
import PurchaseLicense from "./Components/product-page/PurchaseLicense";
import About from "./Components/about-us/About";
import Support from "./Components/support/Support";
import Profile from "./Components/profile/Profile";
import ManageProductPage from "./Components/profile/admin/product-managing/ManageProductPage";
import CreateCompanyAccount from "./Components/profile/register/CreateCompanyAccount";

/**
 * Represents the website content.
 *
 * @returns The website as a JSX element.
 */
export default function App() {
  return (
    <BrowserRouter>
      <Header />

      <main id="main">
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="products" element={<Products />} />
          <Route path="products/:productId" element={<ProductPage />} />
          <Route
            path="products/:productId/purchase-license"
            element={<PurchaseLicense />}
          />
          <Route path="about" element={<About />} />
          <Route path="support" element={<Support />} />
          <Route path="profile/*" element={<Profile />} />
          <Route path="register/*" element={<CreateCompanyAccount />} />

          <Route path="product/*" element={<ProductPage />} />
          <Route
            path="product/purchase-license"
            element={<PurchaseLicense />}
          />
          <Route
            path="product/manage/:productId"
            element={<ManageProductPage />}
          />
          <Route path="product/create" element={<ManageProductPage />} />
        </Routes>
      </main>

      <Footer />
    </BrowserRouter>
  );
}
