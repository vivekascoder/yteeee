import { Box, Button, Flex, Text } from "@chakra-ui/react";
import { useRouter } from "next/navigation";

export default function Navbar() {
  const router = useRouter();
  // debugger;
  return (
    <Flex
      justifyContent={"space-between"}
      py={"4"}
      px={[2, 10, 20]}
      alignItems={"center"}
      position="fixed"
      top={0}
      left={0}
      right={0}
      zIndex="60"
      backgroundColor={"whiteAlpha.500"}
      backdropFilter={"blur(4px)"}
    >
      <Text
        fontWeight={"bold"}
        fontSize={"1.3rem"}
        cursor={"pointer"}
        onClick={() => router.push("/")}
      >
        YTeeee
      </Text>
      <Box display={"flex"}>
        <Button marginRight={4}>Something</Button>
        <Button onClick={() => router.push("/")}>📚 Docs</Button>
      </Box>
    </Flex>
  );
}
