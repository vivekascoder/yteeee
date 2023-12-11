import { Box } from "@chakra-ui/react";
import Head from "next/head";
import React from "react";
import Navbar from "../components/Navbar";

export default function PageLayout(props: React.PropsWithChildren) {
  return (
    <Box style={{ minHeight: "100vh" }}>
      {/* <Box position={"fixed"} bottom="3" left={"4"} zIndex="50">
        <User user={user as UserInfo} />
      </Box>
      <TransactionModal /> */}
      <Head>
        <title>Git DAO</title>
        <meta name="viewport" content="initial-scale=1.0, width=device-width" />
      </Head>
      <Navbar />

      {/* Main content section */}
      <Box maxWidth={"40rem"} pt="20" marginX={"auto"}>
        {props.children}
      </Box>
    </Box>
  );
}
