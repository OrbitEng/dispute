export type Dispute = {
  "version": "0.1.0",
  "name": "dispute",
  "instructions": [
    {
      "name": "openDispute",
      "accounts": [
        {
          "name": "newDispute",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inTransaction",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "we do checks on initial contract",
            "and this contract is cpi auth bound"
          ]
        },
        {
          "name": "buyer",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "seller",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "callerAuth",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "callerProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "threshold",
          "type": "u8"
        }
      ]
    },
    {
      "name": "closeDispute",
      "accounts": [
        {
          "name": "disputeAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "funder",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "callerAuth",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "caller",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "voteDispute",
      "accounts": [
        {
          "name": "disputeAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marketAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "vote",
          "type": {
            "defined": "DisputeSide"
          }
        }
      ]
    },
    {
      "name": "castVerdict",
      "accounts": [
        {
          "name": "disputeAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "disputeAuth",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "disputeProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketAccountsProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "orbitDispute",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "disputeTransaction",
            "type": "publicKey"
          },
          {
            "name": "favor",
            "type": "u64"
          },
          {
            "name": "funder",
            "type": "publicKey"
          },
          {
            "name": "disputeState",
            "type": {
              "defined": "DisputeState"
            }
          },
          {
            "name": "buyer",
            "type": "u64"
          },
          {
            "name": "seller",
            "type": "u64"
          },
          {
            "name": "buyerVotes",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "sellerVotes",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "threshold",
            "type": {
              "defined": "usize"
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "DisputeSide",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Seller"
          },
          {
            "name": "Buyer"
          }
        ]
      }
    },
    {
      "name": "DisputeState",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Open"
          },
          {
            "name": "Closed"
          },
          {
            "name": "Resolved"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "EvenThreshold",
      "msg": "threshold must be an odd number of people"
    },
    {
      "code": 6001,
      "name": "CannotCloseDispute",
      "msg": "can't close dispute"
    },
    {
      "code": 6002,
      "name": "AlreadyVoted",
      "msg": "you've already voted"
    },
    {
      "code": 6003,
      "name": "WrongRemainingAccounts",
      "msg": "wrong remaining accounts"
    }
  ]
};

export const IDL: Dispute = {
  "version": "0.1.0",
  "name": "dispute",
  "instructions": [
    {
      "name": "openDispute",
      "accounts": [
        {
          "name": "newDispute",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "inTransaction",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "we do checks on initial contract",
            "and this contract is cpi auth bound"
          ]
        },
        {
          "name": "buyer",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "seller",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "callerAuth",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "callerProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "threshold",
          "type": "u8"
        }
      ]
    },
    {
      "name": "closeDispute",
      "accounts": [
        {
          "name": "disputeAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "funder",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "callerAuth",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "caller",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "voteDispute",
      "accounts": [
        {
          "name": "disputeAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marketAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "vote",
          "type": {
            "defined": "DisputeSide"
          }
        }
      ]
    },
    {
      "name": "castVerdict",
      "accounts": [
        {
          "name": "disputeAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "disputeAuth",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "disputeProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketAccountsProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "orbitDispute",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "disputeTransaction",
            "type": "publicKey"
          },
          {
            "name": "favor",
            "type": "u64"
          },
          {
            "name": "funder",
            "type": "publicKey"
          },
          {
            "name": "disputeState",
            "type": {
              "defined": "DisputeState"
            }
          },
          {
            "name": "buyer",
            "type": "u64"
          },
          {
            "name": "seller",
            "type": "u64"
          },
          {
            "name": "buyerVotes",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "sellerVotes",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "threshold",
            "type": {
              "defined": "usize"
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "DisputeSide",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Seller"
          },
          {
            "name": "Buyer"
          }
        ]
      }
    },
    {
      "name": "DisputeState",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Open"
          },
          {
            "name": "Closed"
          },
          {
            "name": "Resolved"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "EvenThreshold",
      "msg": "threshold must be an odd number of people"
    },
    {
      "code": 6001,
      "name": "CannotCloseDispute",
      "msg": "can't close dispute"
    },
    {
      "code": 6002,
      "name": "AlreadyVoted",
      "msg": "you've already voted"
    },
    {
      "code": 6003,
      "name": "WrongRemainingAccounts",
      "msg": "wrong remaining accounts"
    }
  ]
};
