import { PublicKey } from "@solana/web3.js";

const anchor = require('@coral-xyz/anchor')

describe('votee', () => {
  const provider = anchor.AnchorProvider.local()
  anchor.setProvider(provider)
  const program = anchor.workspace.Votee;
  let PID: any, CID : any

  it('Initialize and create a poll', async () => {
    const user = provider.wallet

    const [counterPda] = await PublicKey.findProgramAddressSync(
      [Buffer.from("counter")],
      program.programId
    )

    const [registrationsPda] = await PublicKey.findProgramAddressSync(
      [Buffer.from("registrations")],
      program.programId
    )

    let counter
    try {
      counter = await program.account.counter.fetch(counterPda)
      console.log("Counter account already exists with count: ", counter.count.toString())
    } catch (error) {
      console.log("Counter account does not exist, initializing...")
      await program.rpc.initialize({
        accounts: {
          user: user.publicKey,
          counter: counterPda,
          registrations: registrationsPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
    })

      counter = await program.account.counter.fetch(counterPda)
      console.log("Counter account initialized with count: ", counter.count.toString())
    }

    PID = counter.count.add(new anchor.BN(1))
    const [pollPda] = await PublicKey.findProgramAddressSync(
      [PID.toArrayLike(Buffer, "le", 8)],
      program.programId
    )

    const description = `Test Poll #${PID}`
    const start = new anchor.BN(Date.now() / 1000)
    const end = new anchor.BN(Date.now() / 1000 + 86400)

    // Call the createPoll function
    await program.rpc.createPoll(description, start, end, {
      accounts:{
        user: user.publicKey,
        poll: pollPda,
        counter: counterPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    })

    // Verify that the poll was created successfully
    const poll = await program.account.poll.fetch(pollPda)
    console.log('Poll:', poll)
  })

  it('Registers a candidate', async () => {
    const user = provider.wallet

    const [pollPda] = await PublicKey.findProgramAddressSync(
      [PID.toArrayLike(Buffer, "le", 8)],
      program.programId
    )

    const [registrationsPda] = await PublicKey.findProgramAddressSync(
      [Buffer.from("registrations")],
      program.programId
    )

    const regs = await program.account.registrations.fetch(registrationsPda)
    CID = regs.count.add(new anchor.BN(1))

    const candidateName = `Candidate #${CID}`
    const [candidatePda] = await PublicKey.findProgramAddressSync(
      [
        PID.toArrayLike(Buffer, "le", 8),
        CID.toArrayLike(Buffer, "le", 8)
      ],
      program.programId
    )

    // Call the registerCandidate function
    await program.rpc.registerCandidate(PID, candidateName, {
      accounts: {
        user: user.publicKey,
        poll: pollPda,
        candidate: candidatePda,
        registrations: registrationsPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    })
    // Verify that the candidate was registered successfully
    const candidate = await program.account.candidate.fetch(candidatePda)
    console.log('Candidate:', candidate)
  })

  it('Votes for a candidate', async () => {
    const user = provider.wallet

    const [pollPda] = await PublicKey.findProgramAddressSync(
      [PID.toArrayLike(Buffer, "le", 8)],
      program.programId
    )

    const [candidatePda] = await PublicKey.findProgramAddressSync(
      [
        PID.toArrayLike(Buffer, "le", 8),
        CID.toArrayLike(Buffer, "le", 8)
      ],
      program.programId
    )

    const [voterPda] = await PublicKey.findProgramAddressSync(
      [
        Buffer.from("voter"),
        PID.toArrayLike(Buffer, "le", 8),
        user.publicKey.toBuffer(),
      ],
      program.programId
    )

    const [registrationsPda] = await PublicKey.findProgramAddressSync(
      [Buffer.from("registrations")],
      program.programId
    )

    // Call the vote function
    await program.rpc.vote(PID, CID, {
      accounts: {
        user: user.publicKey,
        poll: pollPda,
        candidate: candidatePda,
        voter: voterPda,
        registrations: registrationsPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    })

    // Verify that the vote was cast successfully
    const voter = await program.account.voter.fetch(voterPda)
    console.log('Voter after voting:', voter)

    // Verify that the candidate's vote count was updated
    const candidate = await program.account.candidate.fetch(candidatePda)
    console.log('Candidate votes after voting:', candidate.votes.toString())
  })

})
