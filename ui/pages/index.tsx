import Head from "next/head";
import Image from "next/image";
import { Inter } from "next/font/google";
import styles from "@/styles/Home.module.css";
import { Main } from "next/document";
import Navbar from "@/components/Navbar";
import PageLayout from "@/layouts";
import { Heading } from "@chakra-ui/react";
import SummarizeVideoForm from "@/components/SummarizeVideoForm";

const inter = Inter({ subsets: ["latin"] });

export default function Home() {
  return (
    <PageLayout>
      <Heading>Summarize youtube videos in seconds...</Heading>
      <br />
      <SummarizeVideoForm />
    </PageLayout>
  );
}
