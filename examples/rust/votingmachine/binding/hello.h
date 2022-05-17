// NOTE: We can use https://michael-f-bryan.github.io/rust-ffi-guide/cbindgen.html to generate
// this header automatically from the Rust code.

struct PollingStation;
struct Ballot;
struct InitMessage;
struct SubmitBallotMessage;

void init_stuff();
void hello(char* name);
void run_test();
struct PollingStation* create_polling_station();
struct Ballot* vote(unsigned int voter, _Bool vote_yes);
struct InitMessage* polling_station_init(struct PollingStation* polling_station);
struct SubmitBallotMessage* polling_station_submit(struct PollingStation* polling_station,
                                                   struct Ballot* ballot);
const char* verify_and_get_commit_init(struct InitMessage* init_msg);
const char* verify_and_get_commit(struct SubmitBallotMessage* msg);
void run_remaining(struct PollingStation* polling_station);