import {
  Alert,
  AlertIcon,
  Box,
  Button,
  FormLabel,
  Heading,
  Input,
  Slider,
  SliderFilledTrack,
  SliderThumb,
  SliderTrack,
  Tooltip,
  useToast,
} from "@chakra-ui/react";
import { useFormik } from "formik";

interface ISummarizeVideoForm {
  youtubeUrl: string;
}

export default function SummarizeVideoForm() {
  const toast = useToast();
  const formik = useFormik<ISummarizeVideoForm>({
    initialValues: { youtubeUrl: "" },
    onSubmit: (values) => {},
  });

  // Formik
  //   const formik = useFormik<ICreateDAOFormik>({
  //     initialValues: {
  //       daoTokenName: "",
  //       daoTokenSymbol: "",
  //       tokenSupply: CONFIG.DEFAULT_VALUES.SUPPLY,
  //       minDelay: CONFIG.DEFAULT_VALUES.MIN_DELAY,
  //       quoromPercentage: CONFIG.DEFAULT_VALUES.QUORAM_PERCENTAGE,
  //       votingPeriod: CONFIG.DEFAULT_VALUES.VOTING_PERIOD,
  //       votingDelay: CONFIG.DEFAULT_VALUES.VOTING_DELAY,
  //       adminPercent: CONFIG.DEFAULT_VALUES.ADMIN_PERCENT,
  //     },
  //     validate: (values) => {
  //       // Using a dirty hack here `daoTokenName`
  //       const errors: { [key: string]: string } = {};
  //       // Valudate the values.
  //       if (!repo) {
  //         errors.daoTokenName = "Repo is not selected.";
  //       }
  //       if (values.quoromPercentage <= 0 || values.quoromPercentage > 20) {
  //         errors.daoTokenName =
  //           "Quorom Percentage is not correct please select a value b/w 1 & 20.";
  //       }
  //       if (
  //         values.daoTokenName.length < 2 &&
  //         values.daoTokenName.length < 2 &&
  //         values.daoTokenName.length < 2 &&
  //         values.daoTokenName.length < 2
  //       ) {
  //         errors.daoTokenName =
  //           "The token name and symbol should be atleast 2 char long.";
  //       }
  //       return errors;
  //     },
  //     onSubmit: (values) => {
  //       console.log(values);
  //       write();
  //     },
  //   });

  //   const tokenSupplyWithDecimals = BigNumber.from(
  //     formik.values.tokenSupply || "0"
  //   ).mul(BigNumber.from(ethers.utils.parseEther("1")));

  //   // eslint-disable-next-line
  //   const { data, isLoading, isSuccess, write, error } = useContractWrite({
  //     mode: "recklesslyUnprepared",
  //     addressOrName: CONFIG.CONTRACTS.DAO_FACTORY,
  //     contractInterface: CONFIG.INTERFACES.DAO_FACTORY.abi as ContractInterface,
  //     functionName: "createDAO",
  //     args: [
  //       formik.values.daoTokenName,
  //       formik.values.daoTokenSymbol,
  //       tokenSupplyWithDecimals,
  //       formik.values.adminPercent,
  //       formik.values.minDelay,
  //       formik.values.quoromPercentage,
  //       formik.values.votingPeriod,
  //       formik.values.votingDelay,
  //       repo?.fullName,
  //       repo?.id,
  //     ],
  //     onError: (error) => {
  //       toast({
  //         title: "Error",
  //         description: error.message || "Something went wrong.",
  //         status: "error",
  //         duration: 9000,
  //         isClosable: true,
  //         position: "bottom-right",
  //       });
  //     },
  //     onSuccess(data) {
  //       toast({
  //         title: "Transaction Sent",
  //         description: "Hash: " + data.hash,
  //         status: "success",
  //         duration: 9000,
  //         isClosable: true,
  //         position: "bottom-right",
  //       });
  //     },
  //   });

  return (
    <form>
      <Box
        borderWidth="1px"
        borderRadius="lg"
        boxShadow="1px 1px 3px rgba(0,0,0,0.3)"
        // experimental_spaceY={4}
        p={4}
        my={6}
      >
        <Box>
          <Heading textAlign={"center"}>Summarize</Heading>
        </Box>
        <Box display={"flex"}>
          <Box width="full" mr={3}>
            <FormLabel htmlFor={"youtubeUrl"}>Enter youtube url</FormLabel>
            <Input
              name="youtubeUrl"
              type={"url"}
              value={formik.values.youtubeUrl}
              onChange={formik.handleChange}
              placeholder={"https://www.youtube.com/watch?v=9iqn1HhFJ6c"}
            />
          </Box>
        </Box>
        <Button mt={4} colorScheme={"blue"} isLoading={false} type="submit">
          Summarize
        </Button>
      </Box>
    </form>
  );
}
