const discovery_gmail = {
    "name": "gmail",
    "mtlsRootUrl": "https://gmail.mtls.googleapis.com/",
    "protocol": "rest",
    "discoveryVersion": "v1",
    "icons": {
        "x16": "http://www.google.com/images/icons/product/search-16.gif",
        "x32": "http://www.google.com/images/icons/product/search-32.gif"
    },
    "canonicalName": "Gmail",
    "kind": "discovery#restDescription",
    "servicePath": "",
    "resources": {
        "users": {
            "methods": {
                "getProfile": {
                    "id": "gmail.users.getProfile",
                    "path": "gmail/v1/users/{userId}/profile",
                    "flatPath": "gmail/v1/users/{userId}/profile",
                    "httpMethod": "GET",
                    "parameters": {
                        "userId": {
                            "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                            "default": "me",
                            "location": "path",
                            "required": true,
                            "type": "string"
                        }
                    },
                    "parameterOrder": [
                        "userId"
                    ],
                    "response": {
                        "$ref": "Profile"
                    },
                    "scopes": [
                        "https://mail.google.com/",
                        "https://www.googleapis.com/auth/gmail.compose",
                        "https://www.googleapis.com/auth/gmail.metadata",
                        "https://www.googleapis.com/auth/gmail.modify",
                        "https://www.googleapis.com/auth/gmail.readonly"
                    ],
                    "description": "Gets the current user's Gmail profile."
                },
                "watch": {
                    "id": "gmail.users.watch",
                    "path": "gmail/v1/users/{userId}/watch",
                    "flatPath": "gmail/v1/users/{userId}/watch",
                    "httpMethod": "POST",
                    "parameters": {
                        "userId": {
                            "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                            "default": "me",
                            "location": "path",
                            "required": true,
                            "type": "string"
                        }
                    },
                    "parameterOrder": [
                        "userId"
                    ],
                    "request": {
                        "$ref": "WatchRequest"
                    },
                    "response": {
                        "$ref": "WatchResponse"
                    },
                    "scopes": [
                        "https://mail.google.com/",
                        "https://www.googleapis.com/auth/gmail.metadata",
                        "https://www.googleapis.com/auth/gmail.modify",
                        "https://www.googleapis.com/auth/gmail.readonly"
                    ],
                    "description": "Set up or update a push notification watch on the given user mailbox."
                },
                "stop": {
                    "id": "gmail.users.stop",
                    "path": "gmail/v1/users/{userId}/stop",
                    "flatPath": "gmail/v1/users/{userId}/stop",
                    "httpMethod": "POST",
                    "parameters": {
                        "userId": {
                            "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                            "default": "me",
                            "location": "path",
                            "required": true,
                            "type": "string"
                        }
                    },
                    "parameterOrder": [
                        "userId"
                    ],
                    "scopes": [
                        "https://mail.google.com/",
                        "https://www.googleapis.com/auth/gmail.metadata",
                        "https://www.googleapis.com/auth/gmail.modify",
                        "https://www.googleapis.com/auth/gmail.readonly"
                    ],
                    "description": "Stop receiving push notifications for the given user mailbox."
                }
            },
            "resources": {
                "drafts": {
                    "methods": {
                        "delete": {
                            "id": "gmail.users.drafts.delete",
                            "path": "gmail/v1/users/{userId}/drafts/{id}",
                            "flatPath": "gmail/v1/users/{userId}/drafts/{id}",
                            "httpMethod": "DELETE",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "id": {
                                    "description": "The ID of the draft to delete.",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId",
                                "id"
                            ],
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.addons.current.action.compose",
                                "https://www.googleapis.com/auth/gmail.compose",
                                "https://www.googleapis.com/auth/gmail.modify"
                            ],
                            "description": "Immediately and permanently deletes the specified draft. Does not simply trash it."
                        },
                        "create": {
                            "id": "gmail.users.drafts.create",
                            "path": "gmail/v1/users/{userId}/drafts",
                            "flatPath": "gmail/v1/users/{userId}/drafts",
                            "httpMethod": "POST",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "supportsMediaUpload": true,
                            "mediaUpload": {
                                "accept": [
                                    "message/*"
                                ],
                                "maxSize": "36700160",
                                "protocols": {
                                    "resumable": {
                                        "multipart": true,
                                        "path": "/resumable/upload/gmail/v1/users/{userId}/drafts"
                                    },
                                    "simple": {
                                        "multipart": true,
                                        "path": "/upload/gmail/v1/users/{userId}/drafts"
                                    }
                                }
                            },
                            "request": {
                                "$ref": "Draft"
                            },
                            "response": {
                                "$ref": "Draft"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.addons.current.action.compose",
                                "https://www.googleapis.com/auth/gmail.compose",
                                "https://www.googleapis.com/auth/gmail.modify"
                            ],
                            "description": "Creates a new draft with the `DRAFT` label."
                        },
                        "get": {
                            "id": "gmail.users.drafts.get",
                            "path": "gmail/v1/users/{userId}/drafts/{id}",
                            "flatPath": "gmail/v1/users/{userId}/drafts/{id}",
                            "httpMethod": "GET",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "id": {
                                    "description": "The ID of the draft to retrieve.",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "format": {
                                    "description": "The format to return the draft in.",
                                    "default": "full",
                                    "location": "query",
                                    "type": "string",
                                    "enumDescriptions": [
                                        "Returns only email message ID and labels; does not return the email headers, body, or payload.",
                                        "Returns the full email message data with body content parsed in the `payload` field; the `raw` field is not used. Format cannot be used when accessing the api using the gmail.metadata scope.",
                                        "Returns the full email message data with body content in the `raw` field as a base64url encoded string; the `payload` field is not used. Format cannot be used when accessing the api using the gmail.metadata scope.",
                                        "Returns only email message ID, labels, and email headers."
                                    ],
                                    "enum": [
                                        "minimal",
                                        "full",
                                        "raw",
                                        "metadata"
                                    ]
                                }
                            },
                            "parameterOrder": [
                                "userId",
                                "id"
                            ],
                            "response": {
                                "$ref": "Draft"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.compose",
                                "https://www.googleapis.com/auth/gmail.modify",
                                "https://www.googleapis.com/auth/gmail.readonly"
                            ],
                            "description": "Gets the specified draft."
                        },
                        "list": {
                            "id": "gmail.users.drafts.list",
                            "path": "gmail/v1/users/{userId}/drafts",
                            "flatPath": "gmail/v1/users/{userId}/drafts",
                            "httpMethod": "GET",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "maxResults": {
                                    "description": "Maximum number of drafts to return. This field defaults to 100. The maximum allowed value for this field is 500.",
                                    "default": "100",
                                    "location": "query",
                                    "type": "integer",
                                    "format": "uint32"
                                },
                                "pageToken": {
                                    "description": "Page token to retrieve a specific page of results in the list.",
                                    "location": "query",
                                    "type": "string"
                                },
                                "q": {
                                    "description": "Only return draft messages matching the specified query. Supports the same query format as the Gmail search box. For example, `\"from:someuser@example.com rfc822msgid: is:unread\"`.",
                                    "location": "query",
                                    "type": "string"
                                },
                                "includeSpamTrash": {
                                    "description": "Include drafts from `SPAM` and `TRASH` in the results.",
                                    "default": "false",
                                    "location": "query",
                                    "type": "boolean"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "response": {
                                "$ref": "ListDraftsResponse"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.compose",
                                "https://www.googleapis.com/auth/gmail.modify",
                                "https://www.googleapis.com/auth/gmail.readonly"
                            ],
                            "description": "Lists the drafts in the user's mailbox."
                        },
                        "send": {
                            "id": "gmail.users.drafts.send",
                            "path": "gmail/v1/users/{userId}/drafts/send",
                            "flatPath": "gmail/v1/users/{userId}/drafts/send",
                            "httpMethod": "POST",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "supportsMediaUpload": true,
                            "mediaUpload": {
                                "accept": [
                                    "message/*"
                                ],
                                "maxSize": "36700160",
                                "protocols": {
                                    "resumable": {
                                        "multipart": true,
                                        "path": "/resumable/upload/gmail/v1/users/{userId}/drafts/send"
                                    },
                                    "simple": {
                                        "multipart": true,
                                        "path": "/upload/gmail/v1/users/{userId}/drafts/send"
                                    }
                                }
                            },
                            "request": {
                                "$ref": "Draft"
                            },
                            "response": {
                                "$ref": "Message"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.addons.current.action.compose",
                                "https://www.googleapis.com/auth/gmail.compose",
                                "https://www.googleapis.com/auth/gmail.modify"
                            ],
                            "description": "Sends the specified, existing draft to the recipients in the `To`, `Cc`, and `Bcc` headers."
                        },
                        "update": {
                            "id": "gmail.users.drafts.update",
                            "path": "gmail/v1/users/{userId}/drafts/{id}",
                            "flatPath": "gmail/v1/users/{userId}/drafts/{id}",
                            "httpMethod": "PUT",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "id": {
                                    "description": "The ID of the draft to update.",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId",
                                "id"
                            ],
                            "supportsMediaUpload": true,
                            "mediaUpload": {
                                "accept": [
                                    "message/*"
                                ],
                                "maxSize": "36700160",
                                "protocols": {
                                    "resumable": {
                                        "multipart": true,
                                        "path": "/resumable/upload/gmail/v1/users/{userId}/drafts/{id}"
                                    },
                                    "simple": {
                                        "multipart": true,
                                        "path": "/upload/gmail/v1/users/{userId}/drafts/{id}"
                                    }
                                }
                            },
                            "request": {
                                "$ref": "Draft"
                            },
                            "response": {
                                "$ref": "Draft"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.addons.current.action.compose",
                                "https://www.googleapis.com/auth/gmail.compose",
                                "https://www.googleapis.com/auth/gmail.modify"
                            ],
                            "description": "Replaces a draft's content."
                        }
                    }
                },
                "history": {
                    "methods": {
                        "list": {
                            "id": "gmail.users.history.list",
                            "path": "gmail/v1/users/{userId}/history",
                            "flatPath": "gmail/v1/users/{userId}/history",
                            "httpMethod": "GET",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "maxResults": {
                                    "description": "Maximum number of history records to return. This field defaults to 100. The maximum allowed value for this field is 500.",
                                    "default": "100",
                                    "location": "query",
                                    "type": "integer",
                                    "format": "uint32"
                                },
                                "pageToken": {
                                    "description": "Page token to retrieve a specific page of results in the list.",
                                    "location": "query",
                                    "type": "string"
                                },
                                "startHistoryId": {
                                    "description": "Required. Returns history records after the specified `startHistoryId`. The supplied `startHistoryId` should be obtained from the `historyId` of a message, thread, or previous `list` response. History IDs increase chronologically but are not contiguous with random gaps in between valid IDs. Supplying an invalid or out of date `startHistoryId` typically returns an `HTTP 404` error code. A `historyId` is typically valid for at least a week, but in some rare circumstances may be valid for only a few hours. If you receive an `HTTP 404` error response, your application should perform a full sync. If you receive no `nextPageToken` in the response, there are no updates to retrieve and you can store the returned `historyId` for a future request.",
                                    "location": "query",
                                    "type": "string",
                                    "format": "uint64"
                                },
                                "labelId": {
                                    "description": "Only return messages with a label matching the ID.",
                                    "location": "query",
                                    "type": "string"
                                },
                                "historyTypes": {
                                    "description": "History types to be returned by the function",
                                    "location": "query",
                                    "repeated": true,
                                    "type": "string",
                                    "enumDescriptions": [
                                        "",
                                        "",
                                        "",
                                        ""
                                    ],
                                    "enum": [
                                        "messageAdded",
                                        "messageDeleted",
                                        "labelAdded",
                                        "labelRemoved"
                                    ]
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "response": {
                                "$ref": "ListHistoryResponse"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.metadata",
                                "https://www.googleapis.com/auth/gmail.modify",
                                "https://www.googleapis.com/auth/gmail.readonly"
                            ],
                            "description": "Lists the history of all changes to the given mailbox. History results are returned in chronological order (increasing `historyId`)."
                        }
                    }
                },
                "messages": {
                    "methods": {
                        "trash": {
                            "id": "gmail.users.messages.trash",
                            "path": "gmail/v1/users/{userId}/messages/{id}/trash",
                            "flatPath": "gmail/v1/users/{userId}/messages/{id}/trash",
                            "httpMethod": "POST",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "id": {
                                    "description": "The ID of the message to Trash.",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId",
                                "id"
                            ],
                            "response": {
                                "$ref": "Message"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.modify"
                            ],
                            "description": "Moves the specified message to the trash."
                        },
                        "untrash": {
                            "id": "gmail.users.messages.untrash",
                            "path": "gmail/v1/users/{userId}/messages/{id}/untrash",
                            "flatPath": "gmail/v1/users/{userId}/messages/{id}/untrash",
                            "httpMethod": "POST",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "id": {
                                    "description": "The ID of the message to remove from Trash.",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId",
                                "id"
                            ],
                            "response": {
                                "$ref": "Message"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.modify"
                            ],
                            "description": "Removes the specified message from the trash."
                        },
                        "delete": {
                            "id": "gmail.users.messages.delete",
                            "path": "gmail/v1/users/{userId}/messages/{id}",
                            "flatPath": "gmail/v1/users/{userId}/messages/{id}",
                            "httpMethod": "DELETE",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "id": {
                                    "description": "The ID of the message to delete.",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId",
                                "id"
                            ],
                            "scopes": [
                                "https://mail.google.com/"
                            ],
                            "description": "Immediately and permanently deletes the specified message. This operation cannot be undone. Prefer `messages.trash` instead."
                        },
                        "batchDelete": {
                            "id": "gmail.users.messages.batchDelete",
                            "path": "gmail/v1/users/{userId}/messages/batchDelete",
                            "flatPath": "gmail/v1/users/{userId}/messages/batchDelete",
                            "httpMethod": "POST",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "request": {
                                "$ref": "BatchDeleteMessagesRequest"
                            },
                            "scopes": [
                                "https://mail.google.com/"
                            ],
                            "description": "Deletes many messages by message ID. Provides no guarantees that messages were not already deleted or even existed at all."
                        },
                        "import": {
                            "id": "gmail.users.messages.import",
                            "path": "gmail/v1/users/{userId}/messages/import",
                            "flatPath": "gmail/v1/users/{userId}/messages/import",
                            "httpMethod": "POST",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "internalDateSource": {
                                    "description": "Source for Gmail's internal date of the message.",
                                    "default": "dateHeader",
                                    "location": "query",
                                    "type": "string",
                                    "enumDescriptions": [
                                        "Internal message date set to current time when received by Gmail.",
                                        "Internal message time based on 'Date' header in email, when valid."
                                    ],
                                    "enum": [
                                        "receivedTime",
                                        "dateHeader"
                                    ]
                                },
                                "neverMarkSpam": {
                                    "description": "Ignore the Gmail spam classifier decision and never mark this email as SPAM in the mailbox.",
                                    "default": "false",
                                    "location": "query",
                                    "type": "boolean"
                                },
                                "processForCalendar": {
                                    "description": "Process calendar invites in the email and add any extracted meetings to the Google Calendar for this user.",
                                    "default": "false",
                                    "location": "query",
                                    "type": "boolean"
                                },
                                "deleted": {
                                    "description": "Mark the email as permanently deleted (not TRASH) and only visible in Google Vault to a Vault administrator. Only used for Google Workspace accounts.",
                                    "default": "false",
                                    "location": "query",
                                    "type": "boolean"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "supportsMediaUpload": true,
                            "mediaUpload": {
                                "accept": [
                                    "message/*"
                                ],
                                "maxSize": "52428800",
                                "protocols": {
                                    "resumable": {
                                        "multipart": true,
                                        "path": "/resumable/upload/gmail/v1/users/{userId}/messages/import"
                                    },
                                    "simple": {
                                        "multipart": true,
                                        "path": "/upload/gmail/v1/users/{userId}/messages/import"
                                    }
                                }
                            },
                            "request": {
                                "$ref": "Message"
                            },
                            "response": {
                                "$ref": "Message"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.insert",
                                "https://www.googleapis.com/auth/gmail.modify"
                            ],
                            "description": "Imports a message into only this user's mailbox, with standard email delivery scanning and classification similar to receiving via SMTP. This method doesn't perform SPF checks, so it might not work for some spam messages, such as those attempting to perform domain spoofing. This method does not send a message."
                        },
                        "insert": {
                            "id": "gmail.users.messages.insert",
                            "path": "gmail/v1/users/{userId}/messages",
                            "flatPath": "gmail/v1/users/{userId}/messages",
                            "httpMethod": "POST",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "internalDateSource": {
                                    "description": "Source for Gmail's internal date of the message.",
                                    "default": "receivedTime",
                                    "location": "query",
                                    "type": "string",
                                    "enumDescriptions": [
                                        "Internal message date set to current time when received by Gmail.",
                                        "Internal message time based on 'Date' header in email, when valid."
                                    ],
                                    "enum": [
                                        "receivedTime",
                                        "dateHeader"
                                    ]
                                },
                                "deleted": {
                                    "description": "Mark the email as permanently deleted (not TRASH) and only visible in Google Vault to a Vault administrator. Only used for Google Workspace accounts.",
                                    "default": "false",
                                    "location": "query",
                                    "type": "boolean"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "supportsMediaUpload": true,
                            "mediaUpload": {
                                "accept": [
                                    "message/*"
                                ],
                                "maxSize": "52428800",
                                "protocols": {
                                    "resumable": {
                                        "multipart": true,
                                        "path": "/resumable/upload/gmail/v1/users/{userId}/messages"
                                    },
                                    "simple": {
                                        "multipart": true,
                                        "path": "/upload/gmail/v1/users/{userId}/messages"
                                    }
                                }
                            },
                            "request": {
                                "$ref": "Message"
                            },
                            "response": {
                                "$ref": "Message"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.insert",
                                "https://www.googleapis.com/auth/gmail.modify"
                            ],
                            "description": "Directly inserts a message into only this user's mailbox similar to `IMAP APPEND`, bypassing most scanning and classification. Does not send a message."
                        },
                        "get": {
                            "id": "gmail.users.messages.get",
                            "path": "gmail/v1/users/{userId}/messages/{id}",
                            "flatPath": "gmail/v1/users/{userId}/messages/{id}",
                            "httpMethod": "GET",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "id": {
                                    "description": "The ID of the message to retrieve. This ID is usually retrieved using `messages.list`. The ID is also contained in the result when a message is inserted (`messages.insert`) or imported (`messages.import`).",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "format": {
                                    "description": "The format to return the message in.",
                                    "default": "full",
                                    "location": "query",
                                    "type": "string",
                                    "enumDescriptions": [
                                        "Returns only email message ID and labels; does not return the email headers, body, or payload.",
                                        "Returns the full email message data with body content parsed in the `payload` field; the `raw` field is not used. Format cannot be used when accessing the api using the gmail.metadata scope.",
                                        "Returns the full email message data with body content in the `raw` field as a base64url encoded string; the `payload` field is not used. Format cannot be used when accessing the api using the gmail.metadata scope.",
                                        "Returns only email message ID, labels, and email headers."
                                    ],
                                    "enum": [
                                        "minimal",
                                        "full",
                                        "raw",
                                        "metadata"
                                    ]
                                },
                                "metadataHeaders": {
                                    "description": "When given and format is `METADATA`, only include headers specified.",
                                    "location": "query",
                                    "repeated": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId",
                                "id"
                            ],
                            "response": {
                                "$ref": "Message"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.addons.current.message.action",
                                "https://www.googleapis.com/auth/gmail.addons.current.message.metadata",
                                "https://www.googleapis.com/auth/gmail.addons.current.message.readonly",
                                "https://www.googleapis.com/auth/gmail.metadata",
                                "https://www.googleapis.com/auth/gmail.modify",
                                "https://www.googleapis.com/auth/gmail.readonly"
                            ],
                            "description": "Gets the specified message."
                        },
                        "send": {
                            "id": "gmail.users.messages.send",
                            "path": "gmail/v1/users/{userId}/messages/send",
                            "flatPath": "gmail/v1/users/{userId}/messages/send",
                            "httpMethod": "POST",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "supportsMediaUpload": true,
                            "mediaUpload": {
                                "accept": [
                                    "message/*"
                                ],
                                "maxSize": "36700160",
                                "protocols": {
                                    "resumable": {
                                        "multipart": true,
                                        "path": "/resumable/upload/gmail/v1/users/{userId}/messages/send"
                                    },
                                    "simple": {
                                        "multipart": true,
                                        "path": "/upload/gmail/v1/users/{userId}/messages/send"
                                    }
                                }
                            },
                            "request": {
                                "$ref": "Message"
                            },
                            "response": {
                                "$ref": "Message"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.addons.current.action.compose",
                                "https://www.googleapis.com/auth/gmail.compose",
                                "https://www.googleapis.com/auth/gmail.modify",
                                "https://www.googleapis.com/auth/gmail.send"
                            ],
                            "description": "Sends the specified message to the recipients in the `To`, `Cc`, and `Bcc` headers. For example usage, see [Sending email](https://developers.google.com/gmail/api/guides/sending)."
                        },
                        "list": {
                            "id": "gmail.users.messages.list",
                            "path": "gmail/v1/users/{userId}/messages",
                            "flatPath": "gmail/v1/users/{userId}/messages",
                            "httpMethod": "GET",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "maxResults": {
                                    "description": "Maximum number of messages to return. This field defaults to 100. The maximum allowed value for this field is 500.",
                                    "default": "100",
                                    "location": "query",
                                    "type": "integer",
                                    "format": "uint32"
                                },
                                "pageToken": {
                                    "description": "Page token to retrieve a specific page of results in the list.",
                                    "location": "query",
                                    "type": "string"
                                },
                                "q": {
                                    "description": "Only return messages matching the specified query. Supports the same query format as the Gmail search box. For example, `\"from:someuser@example.com rfc822msgid: is:unread\"`. Parameter cannot be used when accessing the api using the gmail.metadata scope.",
                                    "location": "query",
                                    "type": "string"
                                },
                                "labelIds": {
                                    "description": "Only return messages with labels that match all of the specified label IDs. Messages in a thread might have labels that other messages in the same thread don't have. To learn more, see [Manage labels on messages and threads](https://developers.google.com/gmail/api/guides/labels#manage_labels_on_messages_threads).",
                                    "location": "query",
                                    "repeated": true,
                                    "type": "string"
                                },
                                "includeSpamTrash": {
                                    "description": "Include messages from `SPAM` and `TRASH` in the results.",
                                    "default": "false",
                                    "location": "query",
                                    "type": "boolean"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "response": {
                                "$ref": "ListMessagesResponse"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.metadata",
                                "https://www.googleapis.com/auth/gmail.modify",
                                "https://www.googleapis.com/auth/gmail.readonly"
                            ],
                            "description": "Lists the messages in the user's mailbox."
                        },
                        "modify": {
                            "id": "gmail.users.messages.modify",
                            "path": "gmail/v1/users/{userId}/messages/{id}/modify",
                            "flatPath": "gmail/v1/users/{userId}/messages/{id}/modify",
                            "httpMethod": "POST",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "id": {
                                    "description": "The ID of the message to modify.",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId",
                                "id"
                            ],
                            "request": {
                                "$ref": "ModifyMessageRequest"
                            },
                            "response": {
                                "$ref": "Message"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.modify"
                            ],
                            "description": "Modifies the labels on the specified message."
                        },
                        "batchModify": {
                            "id": "gmail.users.messages.batchModify",
                            "path": "gmail/v1/users/{userId}/messages/batchModify",
                            "flatPath": "gmail/v1/users/{userId}/messages/batchModify",
                            "httpMethod": "POST",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "request": {
                                "$ref": "BatchModifyMessagesRequest"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.modify"
                            ],
                            "description": "Modifies the labels on the specified messages."
                        }
                    },
                    "resources": {
                        "attachments": {
                            "methods": {
                                "get": {
                                    "id": "gmail.users.messages.attachments.get",
                                    "path": "gmail/v1/users/{userId}/messages/{messageId}/attachments/{id}",
                                    "flatPath": "gmail/v1/users/{userId}/messages/{messageId}/attachments/{id}",
                                    "httpMethod": "GET",
                                    "parameters": {
                                        "userId": {
                                            "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        },
                                        "messageId": {
                                            "description": "The ID of the message containing the attachment.",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        },
                                        "id": {
                                            "description": "The ID of the attachment.",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId",
                                        "messageId",
                                        "id"
                                    ],
                                    "response": {
                                        "$ref": "MessagePartBody"
                                    },
                                    "scopes": [
                                        "https://mail.google.com/",
                                        "https://www.googleapis.com/auth/gmail.addons.current.message.action",
                                        "https://www.googleapis.com/auth/gmail.addons.current.message.readonly",
                                        "https://www.googleapis.com/auth/gmail.modify",
                                        "https://www.googleapis.com/auth/gmail.readonly"
                                    ],
                                    "description": "Gets the specified message attachment."
                                }
                            }
                        }
                    }
                },
                "labels": {
                    "methods": {
                        "create": {
                            "id": "gmail.users.labels.create",
                            "path": "gmail/v1/users/{userId}/labels",
                            "flatPath": "gmail/v1/users/{userId}/labels",
                            "httpMethod": "POST",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "request": {
                                "$ref": "Label"
                            },
                            "response": {
                                "$ref": "Label"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.labels",
                                "https://www.googleapis.com/auth/gmail.modify"
                            ],
                            "description": "Creates a new label."
                        },
                        "delete": {
                            "id": "gmail.users.labels.delete",
                            "path": "gmail/v1/users/{userId}/labels/{id}",
                            "flatPath": "gmail/v1/users/{userId}/labels/{id}",
                            "httpMethod": "DELETE",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "id": {
                                    "description": "The ID of the label to delete.",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId",
                                "id"
                            ],
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.labels",
                                "https://www.googleapis.com/auth/gmail.modify"
                            ],
                            "description": "Immediately and permanently deletes the specified label and removes it from any messages and threads that it is applied to."
                        },
                        "get": {
                            "id": "gmail.users.labels.get",
                            "path": "gmail/v1/users/{userId}/labels/{id}",
                            "flatPath": "gmail/v1/users/{userId}/labels/{id}",
                            "httpMethod": "GET",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "id": {
                                    "description": "The ID of the label to retrieve.",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId",
                                "id"
                            ],
                            "response": {
                                "$ref": "Label"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.labels",
                                "https://www.googleapis.com/auth/gmail.metadata",
                                "https://www.googleapis.com/auth/gmail.modify",
                                "https://www.googleapis.com/auth/gmail.readonly"
                            ],
                            "description": "Gets the specified label."
                        },
                        "list": {
                            "id": "gmail.users.labels.list",
                            "path": "gmail/v1/users/{userId}/labels",
                            "flatPath": "gmail/v1/users/{userId}/labels",
                            "httpMethod": "GET",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "response": {
                                "$ref": "ListLabelsResponse"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.labels",
                                "https://www.googleapis.com/auth/gmail.metadata",
                                "https://www.googleapis.com/auth/gmail.modify",
                                "https://www.googleapis.com/auth/gmail.readonly"
                            ],
                            "description": "Lists all labels in the user's mailbox."
                        },
                        "update": {
                            "id": "gmail.users.labels.update",
                            "path": "gmail/v1/users/{userId}/labels/{id}",
                            "flatPath": "gmail/v1/users/{userId}/labels/{id}",
                            "httpMethod": "PUT",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "id": {
                                    "description": "The ID of the label to update.",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId",
                                "id"
                            ],
                            "request": {
                                "$ref": "Label"
                            },
                            "response": {
                                "$ref": "Label"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.labels",
                                "https://www.googleapis.com/auth/gmail.modify"
                            ],
                            "description": "Updates the specified label."
                        },
                        "patch": {
                            "id": "gmail.users.labels.patch",
                            "path": "gmail/v1/users/{userId}/labels/{id}",
                            "flatPath": "gmail/v1/users/{userId}/labels/{id}",
                            "httpMethod": "PATCH",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "id": {
                                    "description": "The ID of the label to update.",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId",
                                "id"
                            ],
                            "request": {
                                "$ref": "Label"
                            },
                            "response": {
                                "$ref": "Label"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.labels",
                                "https://www.googleapis.com/auth/gmail.modify"
                            ],
                            "description": "Patch the specified label."
                        }
                    }
                },
                "threads": {
                    "methods": {
                        "trash": {
                            "id": "gmail.users.threads.trash",
                            "path": "gmail/v1/users/{userId}/threads/{id}/trash",
                            "flatPath": "gmail/v1/users/{userId}/threads/{id}/trash",
                            "httpMethod": "POST",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "id": {
                                    "description": "The ID of the thread to Trash.",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId",
                                "id"
                            ],
                            "response": {
                                "$ref": "Thread"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.modify"
                            ],
                            "description": "Moves the specified thread to the trash. Any messages that belong to the thread are also moved to the trash."
                        },
                        "untrash": {
                            "id": "gmail.users.threads.untrash",
                            "path": "gmail/v1/users/{userId}/threads/{id}/untrash",
                            "flatPath": "gmail/v1/users/{userId}/threads/{id}/untrash",
                            "httpMethod": "POST",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "id": {
                                    "description": "The ID of the thread to remove from Trash.",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId",
                                "id"
                            ],
                            "response": {
                                "$ref": "Thread"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.modify"
                            ],
                            "description": "Removes the specified thread from the trash. Any messages that belong to the thread are also removed from the trash."
                        },
                        "delete": {
                            "id": "gmail.users.threads.delete",
                            "path": "gmail/v1/users/{userId}/threads/{id}",
                            "flatPath": "gmail/v1/users/{userId}/threads/{id}",
                            "httpMethod": "DELETE",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "id": {
                                    "description": "ID of the Thread to delete.",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId",
                                "id"
                            ],
                            "scopes": [
                                "https://mail.google.com/"
                            ],
                            "description": "Immediately and permanently deletes the specified thread. Any messages that belong to the thread are also deleted. This operation cannot be undone. Prefer `threads.trash` instead."
                        },
                        "get": {
                            "id": "gmail.users.threads.get",
                            "path": "gmail/v1/users/{userId}/threads/{id}",
                            "flatPath": "gmail/v1/users/{userId}/threads/{id}",
                            "httpMethod": "GET",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "id": {
                                    "description": "The ID of the thread to retrieve.",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "format": {
                                    "description": "The format to return the messages in.",
                                    "default": "full",
                                    "location": "query",
                                    "type": "string",
                                    "enumDescriptions": [
                                        "Returns the full email message data with body content parsed in the `payload` field; the `raw` field is not used. Format cannot be used when accessing the api using the gmail.metadata scope.",
                                        "Returns only email message IDs, labels, and email headers.",
                                        "Returns only email message IDs and labels; does not return the email headers, body, or payload."
                                    ],
                                    "enum": [
                                        "full",
                                        "metadata",
                                        "minimal"
                                    ]
                                },
                                "metadataHeaders": {
                                    "description": "When given and format is METADATA, only include headers specified.",
                                    "location": "query",
                                    "repeated": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId",
                                "id"
                            ],
                            "response": {
                                "$ref": "Thread"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.addons.current.message.action",
                                "https://www.googleapis.com/auth/gmail.addons.current.message.metadata",
                                "https://www.googleapis.com/auth/gmail.addons.current.message.readonly",
                                "https://www.googleapis.com/auth/gmail.metadata",
                                "https://www.googleapis.com/auth/gmail.modify",
                                "https://www.googleapis.com/auth/gmail.readonly"
                            ],
                            "description": "Gets the specified thread."
                        },
                        "list": {
                            "id": "gmail.users.threads.list",
                            "path": "gmail/v1/users/{userId}/threads",
                            "flatPath": "gmail/v1/users/{userId}/threads",
                            "httpMethod": "GET",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "maxResults": {
                                    "description": "Maximum number of threads to return. This field defaults to 100. The maximum allowed value for this field is 500.",
                                    "default": "100",
                                    "location": "query",
                                    "type": "integer",
                                    "format": "uint32"
                                },
                                "pageToken": {
                                    "description": "Page token to retrieve a specific page of results in the list.",
                                    "location": "query",
                                    "type": "string"
                                },
                                "q": {
                                    "description": "Only return threads matching the specified query. Supports the same query format as the Gmail search box. For example, `\"from:someuser@example.com rfc822msgid: is:unread\"`. Parameter cannot be used when accessing the api using the gmail.metadata scope.",
                                    "location": "query",
                                    "type": "string"
                                },
                                "labelIds": {
                                    "description": "Only return threads with labels that match all of the specified label IDs.",
                                    "location": "query",
                                    "repeated": true,
                                    "type": "string"
                                },
                                "includeSpamTrash": {
                                    "description": "Include threads from `SPAM` and `TRASH` in the results.",
                                    "default": "false",
                                    "location": "query",
                                    "type": "boolean"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "response": {
                                "$ref": "ListThreadsResponse"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.metadata",
                                "https://www.googleapis.com/auth/gmail.modify",
                                "https://www.googleapis.com/auth/gmail.readonly"
                            ],
                            "description": "Lists the threads in the user's mailbox."
                        },
                        "modify": {
                            "id": "gmail.users.threads.modify",
                            "path": "gmail/v1/users/{userId}/threads/{id}/modify",
                            "flatPath": "gmail/v1/users/{userId}/threads/{id}/modify",
                            "httpMethod": "POST",
                            "parameters": {
                                "userId": {
                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                },
                                "id": {
                                    "description": "The ID of the thread to modify.",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId",
                                "id"
                            ],
                            "request": {
                                "$ref": "ModifyThreadRequest"
                            },
                            "response": {
                                "$ref": "Thread"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.modify"
                            ],
                            "description": "Modifies the labels applied to the thread. This applies to all messages in the thread."
                        }
                    }
                },
                "settings": {
                    "methods": {
                        "getImap": {
                            "id": "gmail.users.settings.getImap",
                            "path": "gmail/v1/users/{userId}/settings/imap",
                            "flatPath": "gmail/v1/users/{userId}/settings/imap",
                            "httpMethod": "GET",
                            "parameters": {
                                "userId": {
                                    "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "response": {
                                "$ref": "ImapSettings"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.modify",
                                "https://www.googleapis.com/auth/gmail.readonly",
                                "https://www.googleapis.com/auth/gmail.settings.basic"
                            ],
                            "description": "Gets IMAP settings."
                        },
                        "updateImap": {
                            "id": "gmail.users.settings.updateImap",
                            "path": "gmail/v1/users/{userId}/settings/imap",
                            "flatPath": "gmail/v1/users/{userId}/settings/imap",
                            "httpMethod": "PUT",
                            "parameters": {
                                "userId": {
                                    "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "request": {
                                "$ref": "ImapSettings"
                            },
                            "response": {
                                "$ref": "ImapSettings"
                            },
                            "scopes": [
                                "https://www.googleapis.com/auth/gmail.settings.basic"
                            ],
                            "description": "Updates IMAP settings."
                        },
                        "getPop": {
                            "id": "gmail.users.settings.getPop",
                            "path": "gmail/v1/users/{userId}/settings/pop",
                            "flatPath": "gmail/v1/users/{userId}/settings/pop",
                            "httpMethod": "GET",
                            "parameters": {
                                "userId": {
                                    "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "response": {
                                "$ref": "PopSettings"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.modify",
                                "https://www.googleapis.com/auth/gmail.readonly",
                                "https://www.googleapis.com/auth/gmail.settings.basic"
                            ],
                            "description": "Gets POP settings."
                        },
                        "updatePop": {
                            "id": "gmail.users.settings.updatePop",
                            "path": "gmail/v1/users/{userId}/settings/pop",
                            "flatPath": "gmail/v1/users/{userId}/settings/pop",
                            "httpMethod": "PUT",
                            "parameters": {
                                "userId": {
                                    "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "request": {
                                "$ref": "PopSettings"
                            },
                            "response": {
                                "$ref": "PopSettings"
                            },
                            "scopes": [
                                "https://www.googleapis.com/auth/gmail.settings.basic"
                            ],
                            "description": "Updates POP settings."
                        },
                        "getVacation": {
                            "id": "gmail.users.settings.getVacation",
                            "path": "gmail/v1/users/{userId}/settings/vacation",
                            "flatPath": "gmail/v1/users/{userId}/settings/vacation",
                            "httpMethod": "GET",
                            "parameters": {
                                "userId": {
                                    "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "response": {
                                "$ref": "VacationSettings"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.modify",
                                "https://www.googleapis.com/auth/gmail.readonly",
                                "https://www.googleapis.com/auth/gmail.settings.basic"
                            ],
                            "description": "Gets vacation responder settings."
                        },
                        "updateVacation": {
                            "id": "gmail.users.settings.updateVacation",
                            "path": "gmail/v1/users/{userId}/settings/vacation",
                            "flatPath": "gmail/v1/users/{userId}/settings/vacation",
                            "httpMethod": "PUT",
                            "parameters": {
                                "userId": {
                                    "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "request": {
                                "$ref": "VacationSettings"
                            },
                            "response": {
                                "$ref": "VacationSettings"
                            },
                            "scopes": [
                                "https://www.googleapis.com/auth/gmail.settings.basic"
                            ],
                            "description": "Updates vacation responder settings."
                        },
                        "getLanguage": {
                            "id": "gmail.users.settings.getLanguage",
                            "path": "gmail/v1/users/{userId}/settings/language",
                            "flatPath": "gmail/v1/users/{userId}/settings/language",
                            "httpMethod": "GET",
                            "parameters": {
                                "userId": {
                                    "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "response": {
                                "$ref": "LanguageSettings"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.modify",
                                "https://www.googleapis.com/auth/gmail.readonly",
                                "https://www.googleapis.com/auth/gmail.settings.basic"
                            ],
                            "description": "Gets language settings."
                        },
                        "updateLanguage": {
                            "id": "gmail.users.settings.updateLanguage",
                            "path": "gmail/v1/users/{userId}/settings/language",
                            "flatPath": "gmail/v1/users/{userId}/settings/language",
                            "httpMethod": "PUT",
                            "parameters": {
                                "userId": {
                                    "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "request": {
                                "$ref": "LanguageSettings"
                            },
                            "response": {
                                "$ref": "LanguageSettings"
                            },
                            "scopes": [
                                "https://www.googleapis.com/auth/gmail.settings.basic"
                            ],
                            "description": "Updates language settings. If successful, the return object contains the `displayLanguage` that was saved for the user, which may differ from the value passed into the request. This is because the requested `displayLanguage` may not be directly supported by Gmail but have a close variant that is, and so the variant may be chosen and saved instead."
                        },
                        "getAutoForwarding": {
                            "id": "gmail.users.settings.getAutoForwarding",
                            "path": "gmail/v1/users/{userId}/settings/autoForwarding",
                            "flatPath": "gmail/v1/users/{userId}/settings/autoForwarding",
                            "httpMethod": "GET",
                            "parameters": {
                                "userId": {
                                    "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "response": {
                                "$ref": "AutoForwarding"
                            },
                            "scopes": [
                                "https://mail.google.com/",
                                "https://www.googleapis.com/auth/gmail.modify",
                                "https://www.googleapis.com/auth/gmail.readonly",
                                "https://www.googleapis.com/auth/gmail.settings.basic"
                            ],
                            "description": "Gets the auto-forwarding setting for the specified account."
                        },
                        "updateAutoForwarding": {
                            "id": "gmail.users.settings.updateAutoForwarding",
                            "path": "gmail/v1/users/{userId}/settings/autoForwarding",
                            "flatPath": "gmail/v1/users/{userId}/settings/autoForwarding",
                            "httpMethod": "PUT",
                            "parameters": {
                                "userId": {
                                    "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                    "default": "me",
                                    "location": "path",
                                    "required": true,
                                    "type": "string"
                                }
                            },
                            "parameterOrder": [
                                "userId"
                            ],
                            "request": {
                                "$ref": "AutoForwarding"
                            },
                            "response": {
                                "$ref": "AutoForwarding"
                            },
                            "scopes": [
                                "https://www.googleapis.com/auth/gmail.settings.sharing"
                            ],
                            "description": "Updates the auto-forwarding setting for the specified account. A verified forwarding address must be specified when auto-forwarding is enabled. This method is only available to service account clients that have been delegated domain-wide authority."
                        }
                    },
                    "resources": {
                        "sendAs": {
                            "methods": {
                                "list": {
                                    "id": "gmail.users.settings.sendAs.list",
                                    "path": "gmail/v1/users/{userId}/settings/sendAs",
                                    "flatPath": "gmail/v1/users/{userId}/settings/sendAs",
                                    "httpMethod": "GET",
                                    "parameters": {
                                        "userId": {
                                            "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId"
                                    ],
                                    "response": {
                                        "$ref": "ListSendAsResponse"
                                    },
                                    "scopes": [
                                        "https://mail.google.com/",
                                        "https://www.googleapis.com/auth/gmail.modify",
                                        "https://www.googleapis.com/auth/gmail.readonly",
                                        "https://www.googleapis.com/auth/gmail.settings.basic"
                                    ],
                                    "description": "Lists the send-as aliases for the specified account. The result includes the primary send-as address associated with the account as well as any custom \"from\" aliases."
                                },
                                "get": {
                                    "id": "gmail.users.settings.sendAs.get",
                                    "path": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}",
                                    "flatPath": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}",
                                    "httpMethod": "GET",
                                    "parameters": {
                                        "userId": {
                                            "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        },
                                        "sendAsEmail": {
                                            "description": "The send-as alias to be retrieved.",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId",
                                        "sendAsEmail"
                                    ],
                                    "response": {
                                        "$ref": "SendAs"
                                    },
                                    "scopes": [
                                        "https://mail.google.com/",
                                        "https://www.googleapis.com/auth/gmail.modify",
                                        "https://www.googleapis.com/auth/gmail.readonly",
                                        "https://www.googleapis.com/auth/gmail.settings.basic"
                                    ],
                                    "description": "Gets the specified send-as alias. Fails with an HTTP 404 error if the specified address is not a member of the collection."
                                },
                                "create": {
                                    "id": "gmail.users.settings.sendAs.create",
                                    "path": "gmail/v1/users/{userId}/settings/sendAs",
                                    "flatPath": "gmail/v1/users/{userId}/settings/sendAs",
                                    "httpMethod": "POST",
                                    "parameters": {
                                        "userId": {
                                            "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId"
                                    ],
                                    "request": {
                                        "$ref": "SendAs"
                                    },
                                    "response": {
                                        "$ref": "SendAs"
                                    },
                                    "scopes": [
                                        "https://www.googleapis.com/auth/gmail.settings.sharing"
                                    ],
                                    "description": "Creates a custom \"from\" send-as alias. If an SMTP MSA is specified, Gmail will attempt to connect to the SMTP service to validate the configuration before creating the alias. If ownership verification is required for the alias, a message will be sent to the email address and the resource's verification status will be set to `pending`; otherwise, the resource will be created with verification status set to `accepted`. If a signature is provided, Gmail will sanitize the HTML before saving it with the alias. This method is only available to service account clients that have been delegated domain-wide authority."
                                },
                                "update": {
                                    "id": "gmail.users.settings.sendAs.update",
                                    "path": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}",
                                    "flatPath": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}",
                                    "httpMethod": "PUT",
                                    "parameters": {
                                        "userId": {
                                            "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        },
                                        "sendAsEmail": {
                                            "description": "The send-as alias to be updated.",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId",
                                        "sendAsEmail"
                                    ],
                                    "request": {
                                        "$ref": "SendAs"
                                    },
                                    "response": {
                                        "$ref": "SendAs"
                                    },
                                    "scopes": [
                                        "https://www.googleapis.com/auth/gmail.settings.basic",
                                        "https://www.googleapis.com/auth/gmail.settings.sharing"
                                    ],
                                    "description": "Updates a send-as alias. If a signature is provided, Gmail will sanitize the HTML before saving it with the alias. Addresses other than the primary address for the account can only be updated by service account clients that have been delegated domain-wide authority."
                                },
                                "patch": {
                                    "id": "gmail.users.settings.sendAs.patch",
                                    "path": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}",
                                    "flatPath": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}",
                                    "httpMethod": "PATCH",
                                    "parameters": {
                                        "userId": {
                                            "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        },
                                        "sendAsEmail": {
                                            "description": "The send-as alias to be updated.",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId",
                                        "sendAsEmail"
                                    ],
                                    "request": {
                                        "$ref": "SendAs"
                                    },
                                    "response": {
                                        "$ref": "SendAs"
                                    },
                                    "scopes": [
                                        "https://www.googleapis.com/auth/gmail.settings.basic",
                                        "https://www.googleapis.com/auth/gmail.settings.sharing"
                                    ],
                                    "description": "Patch the specified send-as alias."
                                },
                                "delete": {
                                    "id": "gmail.users.settings.sendAs.delete",
                                    "path": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}",
                                    "flatPath": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}",
                                    "httpMethod": "DELETE",
                                    "parameters": {
                                        "userId": {
                                            "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        },
                                        "sendAsEmail": {
                                            "description": "The send-as alias to be deleted.",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId",
                                        "sendAsEmail"
                                    ],
                                    "scopes": [
                                        "https://www.googleapis.com/auth/gmail.settings.sharing"
                                    ],
                                    "description": "Deletes the specified send-as alias. Revokes any verification that may have been required for using it. This method is only available to service account clients that have been delegated domain-wide authority."
                                },
                                "verify": {
                                    "id": "gmail.users.settings.sendAs.verify",
                                    "path": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}/verify",
                                    "flatPath": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}/verify",
                                    "httpMethod": "POST",
                                    "parameters": {
                                        "userId": {
                                            "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        },
                                        "sendAsEmail": {
                                            "description": "The send-as alias to be verified.",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId",
                                        "sendAsEmail"
                                    ],
                                    "scopes": [
                                        "https://www.googleapis.com/auth/gmail.settings.sharing"
                                    ],
                                    "description": "Sends a verification email to the specified send-as alias address. The verification status must be `pending`. This method is only available to service account clients that have been delegated domain-wide authority."
                                }
                            },
                            "resources": {
                                "smimeInfo": {
                                    "methods": {
                                        "list": {
                                            "id": "gmail.users.settings.sendAs.smimeInfo.list",
                                            "path": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}/smimeInfo",
                                            "flatPath": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}/smimeInfo",
                                            "httpMethod": "GET",
                                            "parameters": {
                                                "userId": {
                                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                                    "default": "me",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                },
                                                "sendAsEmail": {
                                                    "description": "The email address that appears in the \"From:\" header for mail sent using this alias.",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                }
                                            },
                                            "parameterOrder": [
                                                "userId",
                                                "sendAsEmail"
                                            ],
                                            "response": {
                                                "$ref": "ListSmimeInfoResponse"
                                            },
                                            "scopes": [
                                                "https://mail.google.com/",
                                                "https://www.googleapis.com/auth/gmail.modify",
                                                "https://www.googleapis.com/auth/gmail.readonly",
                                                "https://www.googleapis.com/auth/gmail.settings.basic",
                                                "https://www.googleapis.com/auth/gmail.settings.sharing"
                                            ],
                                            "description": "Lists S/MIME configs for the specified send-as alias."
                                        },
                                        "get": {
                                            "id": "gmail.users.settings.sendAs.smimeInfo.get",
                                            "path": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}/smimeInfo/{id}",
                                            "flatPath": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}/smimeInfo/{id}",
                                            "httpMethod": "GET",
                                            "parameters": {
                                                "userId": {
                                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                                    "default": "me",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                },
                                                "sendAsEmail": {
                                                    "description": "The email address that appears in the \"From:\" header for mail sent using this alias.",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                },
                                                "id": {
                                                    "description": "The immutable ID for the SmimeInfo.",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                }
                                            },
                                            "parameterOrder": [
                                                "userId",
                                                "sendAsEmail",
                                                "id"
                                            ],
                                            "response": {
                                                "$ref": "SmimeInfo"
                                            },
                                            "scopes": [
                                                "https://mail.google.com/",
                                                "https://www.googleapis.com/auth/gmail.modify",
                                                "https://www.googleapis.com/auth/gmail.readonly",
                                                "https://www.googleapis.com/auth/gmail.settings.basic",
                                                "https://www.googleapis.com/auth/gmail.settings.sharing"
                                            ],
                                            "description": "Gets the specified S/MIME config for the specified send-as alias."
                                        },
                                        "insert": {
                                            "id": "gmail.users.settings.sendAs.smimeInfo.insert",
                                            "path": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}/smimeInfo",
                                            "flatPath": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}/smimeInfo",
                                            "httpMethod": "POST",
                                            "parameters": {
                                                "userId": {
                                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                                    "default": "me",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                },
                                                "sendAsEmail": {
                                                    "description": "The email address that appears in the \"From:\" header for mail sent using this alias.",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                }
                                            },
                                            "parameterOrder": [
                                                "userId",
                                                "sendAsEmail"
                                            ],
                                            "request": {
                                                "$ref": "SmimeInfo"
                                            },
                                            "response": {
                                                "$ref": "SmimeInfo"
                                            },
                                            "scopes": [
                                                "https://www.googleapis.com/auth/gmail.settings.basic",
                                                "https://www.googleapis.com/auth/gmail.settings.sharing"
                                            ],
                                            "description": "Insert (upload) the given S/MIME config for the specified send-as alias. Note that pkcs12 format is required for the key."
                                        },
                                        "delete": {
                                            "id": "gmail.users.settings.sendAs.smimeInfo.delete",
                                            "path": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}/smimeInfo/{id}",
                                            "flatPath": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}/smimeInfo/{id}",
                                            "httpMethod": "DELETE",
                                            "parameters": {
                                                "userId": {
                                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                                    "default": "me",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                },
                                                "sendAsEmail": {
                                                    "description": "The email address that appears in the \"From:\" header for mail sent using this alias.",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                },
                                                "id": {
                                                    "description": "The immutable ID for the SmimeInfo.",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                }
                                            },
                                            "parameterOrder": [
                                                "userId",
                                                "sendAsEmail",
                                                "id"
                                            ],
                                            "scopes": [
                                                "https://www.googleapis.com/auth/gmail.settings.basic",
                                                "https://www.googleapis.com/auth/gmail.settings.sharing"
                                            ],
                                            "description": "Deletes the specified S/MIME config for the specified send-as alias."
                                        },
                                        "setDefault": {
                                            "id": "gmail.users.settings.sendAs.smimeInfo.setDefault",
                                            "path": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}/smimeInfo/{id}/setDefault",
                                            "flatPath": "gmail/v1/users/{userId}/settings/sendAs/{sendAsEmail}/smimeInfo/{id}/setDefault",
                                            "httpMethod": "POST",
                                            "parameters": {
                                                "userId": {
                                                    "description": "The user's email address. The special value `me` can be used to indicate the authenticated user.",
                                                    "default": "me",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                },
                                                "sendAsEmail": {
                                                    "description": "The email address that appears in the \"From:\" header for mail sent using this alias.",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                },
                                                "id": {
                                                    "description": "The immutable ID for the SmimeInfo.",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                }
                                            },
                                            "parameterOrder": [
                                                "userId",
                                                "sendAsEmail",
                                                "id"
                                            ],
                                            "scopes": [
                                                "https://www.googleapis.com/auth/gmail.settings.basic",
                                                "https://www.googleapis.com/auth/gmail.settings.sharing"
                                            ],
                                            "description": "Sets the default S/MIME config for the specified send-as alias."
                                        }
                                    }
                                }
                            }
                        },
                        "cse": {
                            "resources": {
                                "identities": {
                                    "methods": {
                                        "create": {
                                            "id": "gmail.users.settings.cse.identities.create",
                                            "path": "gmail/v1/users/{userId}/settings/cse/identities",
                                            "flatPath": "gmail/v1/users/{userId}/settings/cse/identities",
                                            "httpMethod": "POST",
                                            "parameters": {
                                                "userId": {
                                                    "description": "The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.",
                                                    "default": "me",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                }
                                            },
                                            "parameterOrder": [
                                                "userId"
                                            ],
                                            "request": {
                                                "$ref": "CseIdentity"
                                            },
                                            "response": {
                                                "$ref": "CseIdentity"
                                            },
                                            "scopes": [
                                                "https://www.googleapis.com/auth/gmail.settings.basic",
                                                "https://www.googleapis.com/auth/gmail.settings.sharing"
                                            ],
                                            "description": "Creates and configures a client-side encryption identity that's authorized to send mail from the user account. Google publishes the S/MIME certificate to a shared domain-wide directory so that people within a Google Workspace organization can encrypt and send mail to the identity."
                                        },
                                        "delete": {
                                            "id": "gmail.users.settings.cse.identities.delete",
                                            "path": "gmail/v1/users/{userId}/settings/cse/identities/{cseEmailAddress}",
                                            "flatPath": "gmail/v1/users/{userId}/settings/cse/identities/{cseEmailAddress}",
                                            "httpMethod": "DELETE",
                                            "parameters": {
                                                "userId": {
                                                    "description": "The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.",
                                                    "default": "me",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                },
                                                "cseEmailAddress": {
                                                    "description": "The primary email address associated with the client-side encryption identity configuration that's removed.",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                }
                                            },
                                            "parameterOrder": [
                                                "userId",
                                                "cseEmailAddress"
                                            ],
                                            "scopes": [
                                                "https://www.googleapis.com/auth/gmail.settings.basic",
                                                "https://www.googleapis.com/auth/gmail.settings.sharing"
                                            ],
                                            "description": "Deletes a client-side encryption identity. The authenticated user can no longer use the identity to send encrypted messages. You cannot restore the identity after you delete it. Instead, use the CreateCseIdentity method to create another identity with the same configuration."
                                        },
                                        "get": {
                                            "id": "gmail.users.settings.cse.identities.get",
                                            "path": "gmail/v1/users/{userId}/settings/cse/identities/{cseEmailAddress}",
                                            "flatPath": "gmail/v1/users/{userId}/settings/cse/identities/{cseEmailAddress}",
                                            "httpMethod": "GET",
                                            "parameters": {
                                                "userId": {
                                                    "description": "The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.",
                                                    "default": "me",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                },
                                                "cseEmailAddress": {
                                                    "description": "The primary email address associated with the client-side encryption identity configuration that's retrieved.",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                }
                                            },
                                            "parameterOrder": [
                                                "userId",
                                                "cseEmailAddress"
                                            ],
                                            "response": {
                                                "$ref": "CseIdentity"
                                            },
                                            "scopes": [
                                                "https://mail.google.com/",
                                                "https://www.googleapis.com/auth/gmail.modify",
                                                "https://www.googleapis.com/auth/gmail.readonly",
                                                "https://www.googleapis.com/auth/gmail.settings.basic",
                                                "https://www.googleapis.com/auth/gmail.settings.sharing"
                                            ],
                                            "description": "Retrieves a client-side encryption identity configuration."
                                        },
                                        "list": {
                                            "id": "gmail.users.settings.cse.identities.list",
                                            "path": "gmail/v1/users/{userId}/settings/cse/identities",
                                            "flatPath": "gmail/v1/users/{userId}/settings/cse/identities",
                                            "httpMethod": "GET",
                                            "parameters": {
                                                "userId": {
                                                    "description": "The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.",
                                                    "default": "me",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                },
                                                "pageToken": {
                                                    "description": "Pagination token indicating which page of identities to return. If the token is not supplied, then the API will return the first page of results.",
                                                    "location": "query",
                                                    "type": "string"
                                                },
                                                "pageSize": {
                                                    "description": "The number of identities to return. If not provided, the page size will default to 20 entries.",
                                                    "default": "20",
                                                    "location": "query",
                                                    "type": "integer",
                                                    "format": "int32"
                                                }
                                            },
                                            "parameterOrder": [
                                                "userId"
                                            ],
                                            "response": {
                                                "$ref": "ListCseIdentitiesResponse"
                                            },
                                            "scopes": [
                                                "https://mail.google.com/",
                                                "https://www.googleapis.com/auth/gmail.modify",
                                                "https://www.googleapis.com/auth/gmail.readonly",
                                                "https://www.googleapis.com/auth/gmail.settings.basic",
                                                "https://www.googleapis.com/auth/gmail.settings.sharing"
                                            ],
                                            "description": "Lists the client-side encrypted identities for an authenticated user."
                                        },
                                        "patch": {
                                            "id": "gmail.users.settings.cse.identities.patch",
                                            "path": "gmail/v1/users/{userId}/settings/cse/identities/{emailAddress}",
                                            "flatPath": "gmail/v1/users/{userId}/settings/cse/identities/{emailAddress}",
                                            "httpMethod": "PATCH",
                                            "parameters": {
                                                "userId": {
                                                    "description": "The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.",
                                                    "default": "me",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                },
                                                "emailAddress": {
                                                    "description": "The email address of the client-side encryption identity to update.",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                }
                                            },
                                            "parameterOrder": [
                                                "userId",
                                                "emailAddress"
                                            ],
                                            "request": {
                                                "$ref": "CseIdentity"
                                            },
                                            "response": {
                                                "$ref": "CseIdentity"
                                            },
                                            "scopes": [
                                                "https://www.googleapis.com/auth/gmail.settings.basic",
                                                "https://www.googleapis.com/auth/gmail.settings.sharing"
                                            ],
                                            "description": "Associates a different key pair with an existing client-side encryption identity. The updated key pair must validate against Google's [S/MIME certificate profiles](https://support.google.com/a/answer/7300887)."
                                        }
                                    }
                                },
                                "keypairs": {
                                    "methods": {
                                        "create": {
                                            "id": "gmail.users.settings.cse.keypairs.create",
                                            "path": "gmail/v1/users/{userId}/settings/cse/keypairs",
                                            "flatPath": "gmail/v1/users/{userId}/settings/cse/keypairs",
                                            "httpMethod": "POST",
                                            "parameters": {
                                                "userId": {
                                                    "description": "The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.",
                                                    "default": "me",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                }
                                            },
                                            "parameterOrder": [
                                                "userId"
                                            ],
                                            "request": {
                                                "$ref": "CseKeyPair"
                                            },
                                            "response": {
                                                "$ref": "CseKeyPair"
                                            },
                                            "scopes": [
                                                "https://www.googleapis.com/auth/gmail.settings.basic",
                                                "https://www.googleapis.com/auth/gmail.settings.sharing"
                                            ],
                                            "description": "Creates and uploads a client-side encryption S/MIME public key certificate chain and private key metadata for the authenticated user."
                                        },
                                        "disable": {
                                            "id": "gmail.users.settings.cse.keypairs.disable",
                                            "path": "gmail/v1/users/{userId}/settings/cse/keypairs/{keyPairId}:disable",
                                            "flatPath": "gmail/v1/users/{userId}/settings/cse/keypairs/{keyPairId}:disable",
                                            "httpMethod": "POST",
                                            "parameters": {
                                                "userId": {
                                                    "description": "The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.",
                                                    "default": "me",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                },
                                                "keyPairId": {
                                                    "description": "The identifier of the key pair to turn off.",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                }
                                            },
                                            "parameterOrder": [
                                                "userId",
                                                "keyPairId"
                                            ],
                                            "request": {
                                                "$ref": "DisableCseKeyPairRequest"
                                            },
                                            "response": {
                                                "$ref": "CseKeyPair"
                                            },
                                            "scopes": [
                                                "https://www.googleapis.com/auth/gmail.settings.basic",
                                                "https://www.googleapis.com/auth/gmail.settings.sharing"
                                            ],
                                            "description": "Turns off a client-side encryption key pair. The authenticated user can no longer use the key pair to decrypt incoming CSE message texts or sign outgoing CSE mail. To regain access, use the EnableCseKeyPair to turn on the key pair. After 30 days, you can permanently delete the key pair by using the ObliterateCseKeyPair method."
                                        },
                                        "enable": {
                                            "id": "gmail.users.settings.cse.keypairs.enable",
                                            "path": "gmail/v1/users/{userId}/settings/cse/keypairs/{keyPairId}:enable",
                                            "flatPath": "gmail/v1/users/{userId}/settings/cse/keypairs/{keyPairId}:enable",
                                            "httpMethod": "POST",
                                            "parameters": {
                                                "userId": {
                                                    "description": "The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.",
                                                    "default": "me",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                },
                                                "keyPairId": {
                                                    "description": "The identifier of the key pair to turn on.",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                }
                                            },
                                            "parameterOrder": [
                                                "userId",
                                                "keyPairId"
                                            ],
                                            "request": {
                                                "$ref": "EnableCseKeyPairRequest"
                                            },
                                            "response": {
                                                "$ref": "CseKeyPair"
                                            },
                                            "scopes": [
                                                "https://www.googleapis.com/auth/gmail.settings.basic",
                                                "https://www.googleapis.com/auth/gmail.settings.sharing"
                                            ],
                                            "description": "Turns on a client-side encryption key pair that was turned off. The key pair becomes active again for any associated client-side encryption identities."
                                        },
                                        "get": {
                                            "id": "gmail.users.settings.cse.keypairs.get",
                                            "path": "gmail/v1/users/{userId}/settings/cse/keypairs/{keyPairId}",
                                            "flatPath": "gmail/v1/users/{userId}/settings/cse/keypairs/{keyPairId}",
                                            "httpMethod": "GET",
                                            "parameters": {
                                                "userId": {
                                                    "description": "The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.",
                                                    "default": "me",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                },
                                                "keyPairId": {
                                                    "description": "The identifier of the key pair to retrieve.",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                }
                                            },
                                            "parameterOrder": [
                                                "userId",
                                                "keyPairId"
                                            ],
                                            "response": {
                                                "$ref": "CseKeyPair"
                                            },
                                            "scopes": [
                                                "https://mail.google.com/",
                                                "https://www.googleapis.com/auth/gmail.modify",
                                                "https://www.googleapis.com/auth/gmail.readonly",
                                                "https://www.googleapis.com/auth/gmail.settings.basic",
                                                "https://www.googleapis.com/auth/gmail.settings.sharing"
                                            ],
                                            "description": "Retrieves an existing client-side encryption key pair."
                                        },
                                        "list": {
                                            "id": "gmail.users.settings.cse.keypairs.list",
                                            "path": "gmail/v1/users/{userId}/settings/cse/keypairs",
                                            "flatPath": "gmail/v1/users/{userId}/settings/cse/keypairs",
                                            "httpMethod": "GET",
                                            "parameters": {
                                                "userId": {
                                                    "description": "The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.",
                                                    "default": "me",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                },
                                                "pageToken": {
                                                    "description": "Pagination token indicating which page of key pairs to return. If the token is not supplied, then the API will return the first page of results.",
                                                    "location": "query",
                                                    "type": "string"
                                                },
                                                "pageSize": {
                                                    "description": "The number of key pairs to return. If not provided, the page size will default to 20 entries.",
                                                    "default": "20",
                                                    "location": "query",
                                                    "type": "integer",
                                                    "format": "int32"
                                                }
                                            },
                                            "parameterOrder": [
                                                "userId"
                                            ],
                                            "response": {
                                                "$ref": "ListCseKeyPairsResponse"
                                            },
                                            "scopes": [
                                                "https://mail.google.com/",
                                                "https://www.googleapis.com/auth/gmail.modify",
                                                "https://www.googleapis.com/auth/gmail.readonly",
                                                "https://www.googleapis.com/auth/gmail.settings.basic",
                                                "https://www.googleapis.com/auth/gmail.settings.sharing"
                                            ],
                                            "description": "Lists client-side encryption key pairs for an authenticated user."
                                        },
                                        "obliterate": {
                                            "id": "gmail.users.settings.cse.keypairs.obliterate",
                                            "path": "gmail/v1/users/{userId}/settings/cse/keypairs/{keyPairId}:obliterate",
                                            "flatPath": "gmail/v1/users/{userId}/settings/cse/keypairs/{keyPairId}:obliterate",
                                            "httpMethod": "POST",
                                            "parameters": {
                                                "userId": {
                                                    "description": "The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.",
                                                    "default": "me",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                },
                                                "keyPairId": {
                                                    "description": "The identifier of the key pair to obliterate.",
                                                    "location": "path",
                                                    "required": true,
                                                    "type": "string"
                                                }
                                            },
                                            "parameterOrder": [
                                                "userId",
                                                "keyPairId"
                                            ],
                                            "request": {
                                                "$ref": "ObliterateCseKeyPairRequest"
                                            },
                                            "scopes": [
                                                "https://www.googleapis.com/auth/gmail.settings.basic",
                                                "https://www.googleapis.com/auth/gmail.settings.sharing"
                                            ],
                                            "description": "Deletes a client-side encryption key pair permanently and immediately. You can only permanently delete key pairs that have been turned off for more than 30 days. To turn off a key pair, use the DisableCseKeyPair method. Gmail can't restore or decrypt any messages that were encrypted by an obliterated key. Authenticated users and Google Workspace administrators lose access to reading the encrypted messages."
                                        }
                                    }
                                }
                            }
                        },
                        "filters": {
                            "methods": {
                                "list": {
                                    "id": "gmail.users.settings.filters.list",
                                    "path": "gmail/v1/users/{userId}/settings/filters",
                                    "flatPath": "gmail/v1/users/{userId}/settings/filters",
                                    "httpMethod": "GET",
                                    "parameters": {
                                        "userId": {
                                            "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId"
                                    ],
                                    "response": {
                                        "$ref": "ListFiltersResponse"
                                    },
                                    "scopes": [
                                        "https://mail.google.com/",
                                        "https://www.googleapis.com/auth/gmail.modify",
                                        "https://www.googleapis.com/auth/gmail.readonly",
                                        "https://www.googleapis.com/auth/gmail.settings.basic"
                                    ],
                                    "description": "Lists the message filters of a Gmail user."
                                },
                                "get": {
                                    "id": "gmail.users.settings.filters.get",
                                    "path": "gmail/v1/users/{userId}/settings/filters/{id}",
                                    "flatPath": "gmail/v1/users/{userId}/settings/filters/{id}",
                                    "httpMethod": "GET",
                                    "parameters": {
                                        "userId": {
                                            "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        },
                                        "id": {
                                            "description": "The ID of the filter to be fetched.",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId",
                                        "id"
                                    ],
                                    "response": {
                                        "$ref": "Filter"
                                    },
                                    "scopes": [
                                        "https://mail.google.com/",
                                        "https://www.googleapis.com/auth/gmail.modify",
                                        "https://www.googleapis.com/auth/gmail.readonly",
                                        "https://www.googleapis.com/auth/gmail.settings.basic"
                                    ],
                                    "description": "Gets a filter."
                                },
                                "create": {
                                    "id": "gmail.users.settings.filters.create",
                                    "path": "gmail/v1/users/{userId}/settings/filters",
                                    "flatPath": "gmail/v1/users/{userId}/settings/filters",
                                    "httpMethod": "POST",
                                    "parameters": {
                                        "userId": {
                                            "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId"
                                    ],
                                    "request": {
                                        "$ref": "Filter"
                                    },
                                    "response": {
                                        "$ref": "Filter"
                                    },
                                    "scopes": [
                                        "https://www.googleapis.com/auth/gmail.settings.basic"
                                    ],
                                    "description": "Creates a filter. Note: you can only create a maximum of 1,000 filters."
                                },
                                "delete": {
                                    "id": "gmail.users.settings.filters.delete",
                                    "path": "gmail/v1/users/{userId}/settings/filters/{id}",
                                    "flatPath": "gmail/v1/users/{userId}/settings/filters/{id}",
                                    "httpMethod": "DELETE",
                                    "parameters": {
                                        "userId": {
                                            "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        },
                                        "id": {
                                            "description": "The ID of the filter to be deleted.",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId",
                                        "id"
                                    ],
                                    "scopes": [
                                        "https://www.googleapis.com/auth/gmail.settings.basic"
                                    ],
                                    "description": "Immediately and permanently deletes the specified filter."
                                }
                            }
                        },
                        "forwardingAddresses": {
                            "methods": {
                                "list": {
                                    "id": "gmail.users.settings.forwardingAddresses.list",
                                    "path": "gmail/v1/users/{userId}/settings/forwardingAddresses",
                                    "flatPath": "gmail/v1/users/{userId}/settings/forwardingAddresses",
                                    "httpMethod": "GET",
                                    "parameters": {
                                        "userId": {
                                            "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId"
                                    ],
                                    "response": {
                                        "$ref": "ListForwardingAddressesResponse"
                                    },
                                    "scopes": [
                                        "https://mail.google.com/",
                                        "https://www.googleapis.com/auth/gmail.modify",
                                        "https://www.googleapis.com/auth/gmail.readonly",
                                        "https://www.googleapis.com/auth/gmail.settings.basic"
                                    ],
                                    "description": "Lists the forwarding addresses for the specified account."
                                },
                                "get": {
                                    "id": "gmail.users.settings.forwardingAddresses.get",
                                    "path": "gmail/v1/users/{userId}/settings/forwardingAddresses/{forwardingEmail}",
                                    "flatPath": "gmail/v1/users/{userId}/settings/forwardingAddresses/{forwardingEmail}",
                                    "httpMethod": "GET",
                                    "parameters": {
                                        "userId": {
                                            "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        },
                                        "forwardingEmail": {
                                            "description": "The forwarding address to be retrieved.",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId",
                                        "forwardingEmail"
                                    ],
                                    "response": {
                                        "$ref": "ForwardingAddress"
                                    },
                                    "scopes": [
                                        "https://mail.google.com/",
                                        "https://www.googleapis.com/auth/gmail.modify",
                                        "https://www.googleapis.com/auth/gmail.readonly",
                                        "https://www.googleapis.com/auth/gmail.settings.basic"
                                    ],
                                    "description": "Gets the specified forwarding address."
                                },
                                "create": {
                                    "id": "gmail.users.settings.forwardingAddresses.create",
                                    "path": "gmail/v1/users/{userId}/settings/forwardingAddresses",
                                    "flatPath": "gmail/v1/users/{userId}/settings/forwardingAddresses",
                                    "httpMethod": "POST",
                                    "parameters": {
                                        "userId": {
                                            "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId"
                                    ],
                                    "request": {
                                        "$ref": "ForwardingAddress"
                                    },
                                    "response": {
                                        "$ref": "ForwardingAddress"
                                    },
                                    "scopes": [
                                        "https://www.googleapis.com/auth/gmail.settings.sharing"
                                    ],
                                    "description": "Creates a forwarding address. If ownership verification is required, a message will be sent to the recipient and the resource's verification status will be set to `pending`; otherwise, the resource will be created with verification status set to `accepted`. This method is only available to service account clients that have been delegated domain-wide authority."
                                },
                                "delete": {
                                    "id": "gmail.users.settings.forwardingAddresses.delete",
                                    "path": "gmail/v1/users/{userId}/settings/forwardingAddresses/{forwardingEmail}",
                                    "flatPath": "gmail/v1/users/{userId}/settings/forwardingAddresses/{forwardingEmail}",
                                    "httpMethod": "DELETE",
                                    "parameters": {
                                        "userId": {
                                            "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        },
                                        "forwardingEmail": {
                                            "description": "The forwarding address to be deleted.",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId",
                                        "forwardingEmail"
                                    ],
                                    "scopes": [
                                        "https://www.googleapis.com/auth/gmail.settings.sharing"
                                    ],
                                    "description": "Deletes the specified forwarding address and revokes any verification that may have been required. This method is only available to service account clients that have been delegated domain-wide authority."
                                }
                            }
                        },
                        "delegates": {
                            "methods": {
                                "list": {
                                    "id": "gmail.users.settings.delegates.list",
                                    "path": "gmail/v1/users/{userId}/settings/delegates",
                                    "flatPath": "gmail/v1/users/{userId}/settings/delegates",
                                    "httpMethod": "GET",
                                    "parameters": {
                                        "userId": {
                                            "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId"
                                    ],
                                    "response": {
                                        "$ref": "ListDelegatesResponse"
                                    },
                                    "scopes": [
                                        "https://mail.google.com/",
                                        "https://www.googleapis.com/auth/gmail.modify",
                                        "https://www.googleapis.com/auth/gmail.readonly",
                                        "https://www.googleapis.com/auth/gmail.settings.basic"
                                    ],
                                    "description": "Lists the delegates for the specified account. This method is only available to service account clients that have been delegated domain-wide authority."
                                },
                                "get": {
                                    "id": "gmail.users.settings.delegates.get",
                                    "path": "gmail/v1/users/{userId}/settings/delegates/{delegateEmail}",
                                    "flatPath": "gmail/v1/users/{userId}/settings/delegates/{delegateEmail}",
                                    "httpMethod": "GET",
                                    "parameters": {
                                        "userId": {
                                            "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        },
                                        "delegateEmail": {
                                            "description": "The email address of the user whose delegate relationship is to be retrieved.",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId",
                                        "delegateEmail"
                                    ],
                                    "response": {
                                        "$ref": "Delegate"
                                    },
                                    "scopes": [
                                        "https://mail.google.com/",
                                        "https://www.googleapis.com/auth/gmail.modify",
                                        "https://www.googleapis.com/auth/gmail.readonly",
                                        "https://www.googleapis.com/auth/gmail.settings.basic"
                                    ],
                                    "description": "Gets the specified delegate. Note that a delegate user must be referred to by their primary email address, and not an email alias. This method is only available to service account clients that have been delegated domain-wide authority."
                                },
                                "create": {
                                    "id": "gmail.users.settings.delegates.create",
                                    "path": "gmail/v1/users/{userId}/settings/delegates",
                                    "flatPath": "gmail/v1/users/{userId}/settings/delegates",
                                    "httpMethod": "POST",
                                    "parameters": {
                                        "userId": {
                                            "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId"
                                    ],
                                    "request": {
                                        "$ref": "Delegate"
                                    },
                                    "response": {
                                        "$ref": "Delegate"
                                    },
                                    "scopes": [
                                        "https://www.googleapis.com/auth/gmail.settings.sharing"
                                    ],
                                    "description": "Adds a delegate with its verification status set directly to `accepted`, without sending any verification email. The delegate user must be a member of the same Google Workspace organization as the delegator user. Gmail imposes limitations on the number of delegates and delegators each user in a Google Workspace organization can have. These limits depend on your organization, but in general each user can have up to 25 delegates and up to 10 delegators. Note that a delegate user must be referred to by their primary email address, and not an email alias. Also note that when a new delegate is created, there may be up to a one minute delay before the new delegate is available for use. This method is only available to service account clients that have been delegated domain-wide authority."
                                },
                                "delete": {
                                    "id": "gmail.users.settings.delegates.delete",
                                    "path": "gmail/v1/users/{userId}/settings/delegates/{delegateEmail}",
                                    "flatPath": "gmail/v1/users/{userId}/settings/delegates/{delegateEmail}",
                                    "httpMethod": "DELETE",
                                    "parameters": {
                                        "userId": {
                                            "description": "User's email address. The special value \"me\" can be used to indicate the authenticated user.",
                                            "default": "me",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        },
                                        "delegateEmail": {
                                            "description": "The email address of the user to be removed as a delegate.",
                                            "location": "path",
                                            "required": true,
                                            "type": "string"
                                        }
                                    },
                                    "parameterOrder": [
                                        "userId",
                                        "delegateEmail"
                                    ],
                                    "scopes": [
                                        "https://www.googleapis.com/auth/gmail.settings.sharing"
                                    ],
                                    "description": "Removes the specified delegate (which can be of any verification status), and revokes any verification that may have been required for using it. Note that a delegate user must be referred to by their primary email address, and not an email alias. This method is only available to service account clients that have been delegated domain-wide authority."
                                }
                            }
                        }
                    }
                }
            }
        }
    },
    "revision": "20240730",
    "description": "The Gmail API lets you view and manage Gmail mailbox data like threads, messages, and labels.",
    "baseUrl": "https://gmail.googleapis.com/",
    "parameters": {
        "access_token": {
            "type": "string",
            "description": "OAuth access token.",
            "location": "query"
        },
        "alt": {
            "type": "string",
            "description": "Data format for response.",
            "default": "json",
            "enum": [
                "json",
                "media",
                "proto"
            ],
            "enumDescriptions": [
                "Responses with Content-Type of application/json",
                "Media download with context-dependent Content-Type",
                "Responses with Content-Type of application/x-protobuf"
            ],
            "location": "query"
        },
        "callback": {
            "type": "string",
            "description": "JSONP",
            "location": "query"
        },
        "fields": {
            "type": "string",
            "description": "Selector specifying which fields to include in a partial response.",
            "location": "query"
        },
        "key": {
            "type": "string",
            "description": "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.",
            "location": "query"
        },
        "oauth_token": {
            "type": "string",
            "description": "OAuth 2.0 token for the current user.",
            "location": "query"
        },
        "prettyPrint": {
            "type": "boolean",
            "description": "Returns response with indentations and line breaks.",
            "default": "true",
            "location": "query"
        },
        "quotaUser": {
            "type": "string",
            "description": "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.",
            "location": "query"
        },
        "upload_protocol": {
            "type": "string",
            "description": "Upload protocol for media (e.g. \"raw\", \"multipart\").",
            "location": "query"
        },
        "uploadType": {
            "type": "string",
            "description": "Legacy upload protocol for media (e.g. \"media\", \"multipart\").",
            "location": "query"
        },
        "$.xgafv": {
            "type": "string",
            "description": "V1 error format.",
            "enum": [
                "1",
                "2"
            ],
            "enumDescriptions": [
                "v1 error format",
                "v2 error format"
            ],
            "location": "query"
        }
    },
    "rootUrl": "https://gmail.googleapis.com/",
    "auth": {
        "oauth2": {
            "scopes": {
                "https://mail.google.com/": {
                    "description": "Read, compose, send, and permanently delete all your email from Gmail"
                },
                "https://www.googleapis.com/auth/gmail.addons.current.action.compose": {
                    "description": "Manage drafts and send emails when you interact with the add-on"
                },
                "https://www.googleapis.com/auth/gmail.addons.current.message.action": {
                    "description": "View your email messages when you interact with the add-on"
                },
                "https://www.googleapis.com/auth/gmail.addons.current.message.metadata": {
                    "description": "View your email message metadata when the add-on is running"
                },
                "https://www.googleapis.com/auth/gmail.addons.current.message.readonly": {
                    "description": "View your email messages when the add-on is running"
                },
                "https://www.googleapis.com/auth/gmail.compose": {
                    "description": "Manage drafts and send emails"
                },
                "https://www.googleapis.com/auth/gmail.insert": {
                    "description": "Add emails into your Gmail mailbox"
                },
                "https://www.googleapis.com/auth/gmail.labels": {
                    "description": "See and edit your email labels"
                },
                "https://www.googleapis.com/auth/gmail.metadata": {
                    "description": "View your email message metadata such as labels and headers, but not the email body"
                },
                "https://www.googleapis.com/auth/gmail.modify": {
                    "description": "Read, compose, and send emails from your Gmail account"
                },
                "https://www.googleapis.com/auth/gmail.readonly": {
                    "description": "View your email messages and settings"
                },
                "https://www.googleapis.com/auth/gmail.send": {
                    "description": "Send email on your behalf"
                },
                "https://www.googleapis.com/auth/gmail.settings.basic": {
                    "description": "See, edit, create, or change your email settings and filters in Gmail"
                },
                "https://www.googleapis.com/auth/gmail.settings.sharing": {
                    "description": "Manage your sensitive mail settings, including who can manage your mail"
                }
            }
        }
    },
    "schemas": {
        "Draft": {
            "id": "Draft",
            "description": "A draft email in the user's mailbox.",
            "type": "object",
            "properties": {
                "id": {
                    "description": "The immutable ID of the draft.",
                    "annotations": {
                        "required": [
                            "gmail.users.drafts.send"
                        ]
                    },
                    "type": "string"
                },
                "message": {
                    "description": "The message content of the draft.",
                    "$ref": "Message"
                }
            }
        },
        "Message": {
            "id": "Message",
            "description": "An email message.",
            "type": "object",
            "properties": {
                "id": {
                    "description": "The immutable ID of the message.",
                    "type": "string"
                },
                "threadId": {
                    "description": "The ID of the thread the message belongs to. To add a message or draft to a thread, the following criteria must be met: 1. The requested `threadId` must be specified on the `Message` or `Draft.Message` you supply with your request. 2. The `References` and `In-Reply-To` headers must be set in compliance with the [RFC 2822](https://tools.ietf.org/html/rfc2822) standard. 3. The `Subject` headers must match. ",
                    "type": "string"
                },
                "labelIds": {
                    "description": "List of IDs of labels applied to this message.",
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                },
                "snippet": {
                    "description": "A short part of the message text.",
                    "type": "string"
                },
                "historyId": {
                    "description": "The ID of the last history record that modified this message.",
                    "type": "string",
                    "format": "uint64"
                },
                "internalDate": {
                    "description": "The internal message creation timestamp (epoch ms), which determines ordering in the inbox. For normal SMTP-received email, this represents the time the message was originally accepted by Google, which is more reliable than the `Date` header. However, for API-migrated mail, it can be configured by client to be based on the `Date` header.",
                    "type": "string",
                    "format": "int64"
                },
                "payload": {
                    "description": "The parsed email structure in the message parts.",
                    "$ref": "MessagePart"
                },
                "sizeEstimate": {
                    "description": "Estimated size in bytes of the message.",
                    "type": "integer",
                    "format": "int32"
                },
                "raw": {
                    "description": "The entire email message in an RFC 2822 formatted and base64url encoded string. Returned in `messages.get` and `drafts.get` responses when the `format=RAW` parameter is supplied.",
                    "annotations": {
                        "required": [
                            "gmail.users.drafts.create",
                            "gmail.users.drafts.update",
                            "gmail.users.messages.insert",
                            "gmail.users.messages.send"
                        ]
                    },
                    "type": "string",
                    "format": "byte"
                }
            }
        },
        "MessagePart": {
            "id": "MessagePart",
            "description": "A single MIME message part.",
            "type": "object",
            "properties": {
                "partId": {
                    "description": "The immutable ID of the message part.",
                    "type": "string"
                },
                "mimeType": {
                    "description": "The MIME type of the message part.",
                    "type": "string"
                },
                "filename": {
                    "description": "The filename of the attachment. Only present if this message part represents an attachment.",
                    "type": "string"
                },
                "headers": {
                    "description": "List of headers on this message part. For the top-level message part, representing the entire message payload, it will contain the standard RFC 2822 email headers such as `To`, `From`, and `Subject`.",
                    "type": "array",
                    "items": {
                        "$ref": "MessagePartHeader"
                    }
                },
                "body": {
                    "description": "The message part body for this part, which may be empty for container MIME message parts.",
                    "$ref": "MessagePartBody"
                },
                "parts": {
                    "description": "The child MIME message parts of this part. This only applies to container MIME message parts, for example `multipart/*`. For non- container MIME message part types, such as `text/plain`, this field is empty. For more information, see RFC 1521.",
                    "type": "array",
                    "items": {
                        "$ref": "MessagePart"
                    }
                }
            }
        },
        "MessagePartHeader": {
            "id": "MessagePartHeader",
            "type": "object",
            "properties": {
                "name": {
                    "description": "The name of the header before the `:` separator. For example, `To`.",
                    "type": "string"
                },
                "value": {
                    "description": "The value of the header after the `:` separator. For example, `someuser@example.com`.",
                    "type": "string"
                }
            }
        },
        "MessagePartBody": {
            "id": "MessagePartBody",
            "description": "The body of a single MIME message part.",
            "type": "object",
            "properties": {
                "attachmentId": {
                    "description": "When present, contains the ID of an external attachment that can be retrieved in a separate `messages.attachments.get` request. When not present, the entire content of the message part body is contained in the data field.",
                    "type": "string"
                },
                "size": {
                    "description": "Number of bytes for the message part data (encoding notwithstanding).",
                    "type": "integer",
                    "format": "int32"
                },
                "data": {
                    "description": "The body data of a MIME message part as a base64url encoded string. May be empty for MIME container types that have no message body or when the body data is sent as a separate attachment. An attachment ID is present if the body data is contained in a separate attachment.",
                    "type": "string",
                    "format": "byte"
                }
            }
        },
        "ListDraftsResponse": {
            "id": "ListDraftsResponse",
            "type": "object",
            "properties": {
                "drafts": {
                    "description": "List of drafts. Note that the `Message` property in each `Draft` resource only contains an `id` and a `threadId`. The messages.get method can fetch additional message details.",
                    "type": "array",
                    "items": {
                        "$ref": "Draft"
                    }
                },
                "nextPageToken": {
                    "description": "Token to retrieve the next page of results in the list.",
                    "type": "string"
                },
                "resultSizeEstimate": {
                    "description": "Estimated total number of results.",
                    "type": "integer",
                    "format": "uint32"
                }
            }
        },
        "ListHistoryResponse": {
            "id": "ListHistoryResponse",
            "type": "object",
            "properties": {
                "history": {
                    "description": "List of history records. Any `messages` contained in the response will typically only have `id` and `threadId` fields populated.",
                    "type": "array",
                    "items": {
                        "$ref": "History"
                    }
                },
                "nextPageToken": {
                    "description": "Page token to retrieve the next page of results in the list.",
                    "type": "string"
                },
                "historyId": {
                    "description": "The ID of the mailbox's current history record.",
                    "type": "string",
                    "format": "uint64"
                }
            }
        },
        "History": {
            "id": "History",
            "description": "A record of a change to the user's mailbox. Each history change may affect multiple messages in multiple ways.",
            "type": "object",
            "properties": {
                "id": {
                    "description": "The mailbox sequence ID.",
                    "type": "string",
                    "format": "uint64"
                },
                "messages": {
                    "description": "List of messages changed in this history record. The fields for specific change types, such as `messagesAdded` may duplicate messages in this field. We recommend using the specific change-type fields instead of this.",
                    "type": "array",
                    "items": {
                        "$ref": "Message"
                    }
                },
                "messagesAdded": {
                    "description": "Messages added to the mailbox in this history record.",
                    "type": "array",
                    "items": {
                        "$ref": "HistoryMessageAdded"
                    }
                },
                "messagesDeleted": {
                    "description": "Messages deleted (not Trashed) from the mailbox in this history record.",
                    "type": "array",
                    "items": {
                        "$ref": "HistoryMessageDeleted"
                    }
                },
                "labelsAdded": {
                    "description": "Labels added to messages in this history record.",
                    "type": "array",
                    "items": {
                        "$ref": "HistoryLabelAdded"
                    }
                },
                "labelsRemoved": {
                    "description": "Labels removed from messages in this history record.",
                    "type": "array",
                    "items": {
                        "$ref": "HistoryLabelRemoved"
                    }
                }
            }
        },
        "HistoryMessageAdded": {
            "id": "HistoryMessageAdded",
            "type": "object",
            "properties": {
                "message": {
                    "$ref": "Message"
                }
            }
        },
        "HistoryMessageDeleted": {
            "id": "HistoryMessageDeleted",
            "type": "object",
            "properties": {
                "message": {
                    "$ref": "Message"
                }
            }
        },
        "HistoryLabelAdded": {
            "id": "HistoryLabelAdded",
            "type": "object",
            "properties": {
                "message": {
                    "$ref": "Message"
                },
                "labelIds": {
                    "description": "Label IDs added to the message.",
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                }
            }
        },
        "HistoryLabelRemoved": {
            "id": "HistoryLabelRemoved",
            "type": "object",
            "properties": {
                "message": {
                    "$ref": "Message"
                },
                "labelIds": {
                    "description": "Label IDs removed from the message.",
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                }
            }
        },
        "BatchDeleteMessagesRequest": {
            "id": "BatchDeleteMessagesRequest",
            "type": "object",
            "properties": {
                "ids": {
                    "description": "The IDs of the messages to delete.",
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                }
            }
        },
        "ListMessagesResponse": {
            "id": "ListMessagesResponse",
            "type": "object",
            "properties": {
                "messages": {
                    "description": "List of messages. Note that each message resource contains only an `id` and a `threadId`. Additional message details can be fetched using the messages.get method.",
                    "type": "array",
                    "items": {
                        "$ref": "Message"
                    }
                },
                "nextPageToken": {
                    "description": "Token to retrieve the next page of results in the list.",
                    "type": "string"
                },
                "resultSizeEstimate": {
                    "description": "Estimated total number of results.",
                    "type": "integer",
                    "format": "uint32"
                }
            }
        },
        "ModifyMessageRequest": {
            "id": "ModifyMessageRequest",
            "type": "object",
            "properties": {
                "addLabelIds": {
                    "description": "A list of IDs of labels to add to this message. You can add up to 100 labels with each update.",
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                },
                "removeLabelIds": {
                    "description": "A list IDs of labels to remove from this message. You can remove up to 100 labels with each update.",
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                }
            }
        },
        "BatchModifyMessagesRequest": {
            "id": "BatchModifyMessagesRequest",
            "type": "object",
            "properties": {
                "ids": {
                    "description": "The IDs of the messages to modify. There is a limit of 1000 ids per request.",
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                },
                "addLabelIds": {
                    "description": "A list of label IDs to add to messages.",
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                },
                "removeLabelIds": {
                    "description": "A list of label IDs to remove from messages.",
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                }
            }
        },
        "Label": {
            "id": "Label",
            "description": "Labels are used to categorize messages and threads within the user's mailbox. The maximum number of labels supported for a user's mailbox is 10,000.",
            "type": "object",
            "properties": {
                "id": {
                    "description": "The immutable ID of the label.",
                    "annotations": {
                        "required": [
                            "gmail.users.labels.update"
                        ]
                    },
                    "type": "string"
                },
                "name": {
                    "description": "The display name of the label.",
                    "annotations": {
                        "required": [
                            "gmail.users.labels.create",
                            "gmail.users.labels.update"
                        ]
                    },
                    "type": "string"
                },
                "messageListVisibility": {
                    "description": "The visibility of messages with this label in the message list in the Gmail web interface.",
                    "annotations": {
                        "required": [
                            "gmail.users.labels.create",
                            "gmail.users.labels.update"
                        ]
                    },
                    "type": "string",
                    "enumDescriptions": [
                        "Show the label in the message list.",
                        "Do not show the label in the message list."
                    ],
                    "enum": [
                        "show",
                        "hide"
                    ]
                },
                "labelListVisibility": {
                    "description": "The visibility of the label in the label list in the Gmail web interface.",
                    "annotations": {
                        "required": [
                            "gmail.users.labels.create",
                            "gmail.users.labels.update"
                        ]
                    },
                    "type": "string",
                    "enumDescriptions": [
                        "Show the label in the label list.",
                        "Show the label if there are any unread messages with that label.",
                        "Do not show the label in the label list."
                    ],
                    "enum": [
                        "labelShow",
                        "labelShowIfUnread",
                        "labelHide"
                    ]
                },
                "type": {
                    "description": "The owner type for the label. User labels are created by the user and can be modified and deleted by the user and can be applied to any message or thread. System labels are internally created and cannot be added, modified, or deleted. System labels may be able to be applied to or removed from messages and threads under some circumstances but this is not guaranteed. For example, users can apply and remove the `INBOX` and `UNREAD` labels from messages and threads, but cannot apply or remove the `DRAFTS` or `SENT` labels from messages or threads.",
                    "type": "string",
                    "enumDescriptions": [
                        "Labels created by Gmail.",
                        "Custom labels created by the user or application."
                    ],
                    "enum": [
                        "system",
                        "user"
                    ]
                },
                "messagesTotal": {
                    "description": "The total number of messages with the label.",
                    "type": "integer",
                    "format": "int32"
                },
                "messagesUnread": {
                    "description": "The number of unread messages with the label.",
                    "type": "integer",
                    "format": "int32"
                },
                "threadsTotal": {
                    "description": "The total number of threads with the label.",
                    "type": "integer",
                    "format": "int32"
                },
                "threadsUnread": {
                    "description": "The number of unread threads with the label.",
                    "type": "integer",
                    "format": "int32"
                },
                "color": {
                    "description": "The color to assign to the label. Color is only available for labels that have their `type` set to `user`.",
                    "$ref": "LabelColor"
                }
            }
        },
        "LabelColor": {
            "id": "LabelColor",
            "type": "object",
            "properties": {
                "textColor": {
                    "description": "The text color of the label, represented as hex string. This field is required in order to set the color of a label. Only the following predefined set of color values are allowed: \\#000000, #434343, #666666, #999999, #cccccc, #efefef, #f3f3f3, #ffffff, \\#fb4c2f, #ffad47, #fad165, #16a766, #43d692, #4a86e8, #a479e2, #f691b3, \\#f6c5be, #ffe6c7, #fef1d1, #b9e4d0, #c6f3de, #c9daf8, #e4d7f5, #fcdee8, \\#efa093, #ffd6a2, #fce8b3, #89d3b2, #a0eac9, #a4c2f4, #d0bcf1, #fbc8d9, \\#e66550, #ffbc6b, #fcda83, #44b984, #68dfa9, #6d9eeb, #b694e8, #f7a7c0, \\#cc3a21, #eaa041, #f2c960, #149e60, #3dc789, #3c78d8, #8e63ce, #e07798, \\#ac2b16, #cf8933, #d5ae49, #0b804b, #2a9c68, #285bac, #653e9b, #b65775, \\#822111, #a46a21, #aa8831, #076239, #1a764d, #1c4587, #41236d, #83334c \\#464646, #e7e7e7, #0d3472, #b6cff5, #0d3b44, #98d7e4, #3d188e, #e3d7ff, \\#711a36, #fbd3e0, #8a1c0a, #f2b2a8, #7a2e0b, #ffc8af, #7a4706, #ffdeb5, \\#594c05, #fbe983, #684e07, #fdedc1, #0b4f30, #b3efd3, #04502e, #a2dcc1, \\#c2c2c2, #4986e7, #2da2bb, #b99aff, #994a64, #f691b2, #ff7537, #ffad46, \\#662e37, #ebdbde, #cca6ac, #094228, #42d692, #16a765",
                    "type": "string"
                },
                "backgroundColor": {
                    "description": "The background color represented as hex string #RRGGBB (ex #000000). This field is required in order to set the color of a label. Only the following predefined set of color values are allowed: \\#000000, #434343, #666666, #999999, #cccccc, #efefef, #f3f3f3, #ffffff, \\#fb4c2f, #ffad47, #fad165, #16a766, #43d692, #4a86e8, #a479e2, #f691b3, \\#f6c5be, #ffe6c7, #fef1d1, #b9e4d0, #c6f3de, #c9daf8, #e4d7f5, #fcdee8, \\#efa093, #ffd6a2, #fce8b3, #89d3b2, #a0eac9, #a4c2f4, #d0bcf1, #fbc8d9, \\#e66550, #ffbc6b, #fcda83, #44b984, #68dfa9, #6d9eeb, #b694e8, #f7a7c0, \\#cc3a21, #eaa041, #f2c960, #149e60, #3dc789, #3c78d8, #8e63ce, #e07798, \\#ac2b16, #cf8933, #d5ae49, #0b804b, #2a9c68, #285bac, #653e9b, #b65775, \\#822111, #a46a21, #aa8831, #076239, #1a764d, #1c4587, #41236d, #83334c \\#464646, #e7e7e7, #0d3472, #b6cff5, #0d3b44, #98d7e4, #3d188e, #e3d7ff, \\#711a36, #fbd3e0, #8a1c0a, #f2b2a8, #7a2e0b, #ffc8af, #7a4706, #ffdeb5, \\#594c05, #fbe983, #684e07, #fdedc1, #0b4f30, #b3efd3, #04502e, #a2dcc1, \\#c2c2c2, #4986e7, #2da2bb, #b99aff, #994a64, #f691b2, #ff7537, #ffad46, \\#662e37, #ebdbde, #cca6ac, #094228, #42d692, #16a765",
                    "type": "string"
                }
            }
        },
        "ListLabelsResponse": {
            "id": "ListLabelsResponse",
            "type": "object",
            "properties": {
                "labels": {
                    "description": "List of labels. Note that each label resource only contains an `id`, `name`, `messageListVisibility`, `labelListVisibility`, and `type`. The labels.get method can fetch additional label details.",
                    "type": "array",
                    "items": {
                        "$ref": "Label"
                    }
                }
            }
        },
        "Profile": {
            "id": "Profile",
            "description": "Profile for a Gmail user.",
            "type": "object",
            "properties": {
                "emailAddress": {
                    "description": "The user's email address.",
                    "type": "string"
                },
                "messagesTotal": {
                    "description": "The total number of messages in the mailbox.",
                    "type": "integer",
                    "format": "int32"
                },
                "threadsTotal": {
                    "description": "The total number of threads in the mailbox.",
                    "type": "integer",
                    "format": "int32"
                },
                "historyId": {
                    "description": "The ID of the mailbox's current history record.",
                    "type": "string",
                    "format": "uint64"
                }
            }
        },
        "WatchRequest": {
            "id": "WatchRequest",
            "description": "Set up or update a new push notification watch on this user's mailbox.",
            "type": "object",
            "properties": {
                "labelIds": {
                    "description": "List of label_ids to restrict notifications about. By default, if unspecified, all changes are pushed out. If specified then dictates which labels are required for a push notification to be generated.",
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                },
                "labelFilterAction": {
                    "description": "Filtering behavior of `labelIds list` specified. This field is deprecated because it caused incorrect behavior in some cases; use `label_filter_behavior` instead.",
                    "deprecated": true,
                    "type": "string",
                    "enumDescriptions": [
                        "Only get push notifications for message changes relating to labelIds specified.",
                        "Get push notifications for all message changes except those relating to labelIds specified."
                    ],
                    "enum": [
                        "include",
                        "exclude"
                    ]
                },
                "labelFilterBehavior": {
                    "description": "Filtering behavior of `labelIds list` specified. This field replaces `label_filter_action`; if set, `label_filter_action` is ignored.",
                    "type": "string",
                    "enumDescriptions": [
                        "Only get push notifications for message changes relating to labelIds specified.",
                        "Get push notifications for all message changes except those relating to labelIds specified."
                    ],
                    "enum": [
                        "include",
                        "exclude"
                    ]
                },
                "topicName": {
                    "description": "A fully qualified Google Cloud Pub/Sub API topic name to publish the events to. This topic name **must** already exist in Cloud Pub/Sub and you **must** have already granted gmail \"publish\" permission on it. For example, \"projects/my-project-identifier/topics/my-topic-name\" (using the Cloud Pub/Sub \"v1\" topic naming format). Note that the \"my-project-identifier\" portion must exactly match your Google developer project id (the one executing this watch request).",
                    "type": "string"
                }
            }
        },
        "WatchResponse": {
            "id": "WatchResponse",
            "description": "Push notification watch response.",
            "type": "object",
            "properties": {
                "historyId": {
                    "description": "The ID of the mailbox's current history record.",
                    "type": "string",
                    "format": "uint64"
                },
                "expiration": {
                    "description": "When Gmail will stop sending notifications for mailbox updates (epoch millis). Call `watch` again before this time to renew the watch.",
                    "type": "string",
                    "format": "int64"
                }
            }
        },
        "Thread": {
            "id": "Thread",
            "description": "A collection of messages representing a conversation.",
            "type": "object",
            "properties": {
                "id": {
                    "description": "The unique ID of the thread.",
                    "type": "string"
                },
                "snippet": {
                    "description": "A short part of the message text.",
                    "type": "string"
                },
                "historyId": {
                    "description": "The ID of the last history record that modified this thread.",
                    "type": "string",
                    "format": "uint64"
                },
                "messages": {
                    "description": "The list of messages in the thread.",
                    "type": "array",
                    "items": {
                        "$ref": "Message"
                    }
                }
            }
        },
        "ListThreadsResponse": {
            "id": "ListThreadsResponse",
            "type": "object",
            "properties": {
                "threads": {
                    "description": "List of threads. Note that each thread resource does not contain a list of `messages`. The list of `messages` for a given thread can be fetched using the threads.get method.",
                    "type": "array",
                    "items": {
                        "$ref": "Thread"
                    }
                },
                "nextPageToken": {
                    "description": "Page token to retrieve the next page of results in the list.",
                    "type": "string"
                },
                "resultSizeEstimate": {
                    "description": "Estimated total number of results.",
                    "type": "integer",
                    "format": "uint32"
                }
            }
        },
        "ModifyThreadRequest": {
            "id": "ModifyThreadRequest",
            "type": "object",
            "properties": {
                "addLabelIds": {
                    "description": "A list of IDs of labels to add to this thread. You can add up to 100 labels with each update.",
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                },
                "removeLabelIds": {
                    "description": "A list of IDs of labels to remove from this thread. You can remove up to 100 labels with each update.",
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                }
            }
        },
        "ListSendAsResponse": {
            "id": "ListSendAsResponse",
            "description": "Response for the ListSendAs method.",
            "type": "object",
            "properties": {
                "sendAs": {
                    "description": "List of send-as aliases.",
                    "type": "array",
                    "items": {
                        "$ref": "SendAs"
                    }
                }
            }
        },
        "SendAs": {
            "id": "SendAs",
            "description": "Settings associated with a send-as alias, which can be either the primary login address associated with the account or a custom \"from\" address. Send-as aliases correspond to the \"Send Mail As\" feature in the web interface.",
            "type": "object",
            "properties": {
                "sendAsEmail": {
                    "description": "The email address that appears in the \"From:\" header for mail sent using this alias. This is read-only for all operations except create.",
                    "type": "string"
                },
                "displayName": {
                    "description": "A name that appears in the \"From:\" header for mail sent using this alias. For custom \"from\" addresses, when this is empty, Gmail will populate the \"From:\" header with the name that is used for the primary address associated with the account. If the admin has disabled the ability for users to update their name format, requests to update this field for the primary login will silently fail.",
                    "type": "string"
                },
                "replyToAddress": {
                    "description": "An optional email address that is included in a \"Reply-To:\" header for mail sent using this alias. If this is empty, Gmail will not generate a \"Reply-To:\" header.",
                    "type": "string"
                },
                "signature": {
                    "description": "An optional HTML signature that is included in messages composed with this alias in the Gmail web UI. This signature is added to new emails only.",
                    "type": "string"
                },
                "isPrimary": {
                    "description": "Whether this address is the primary address used to login to the account. Every Gmail account has exactly one primary address, and it cannot be deleted from the collection of send-as aliases. This field is read-only.",
                    "type": "boolean"
                },
                "isDefault": {
                    "description": "Whether this address is selected as the default \"From:\" address in situations such as composing a new message or sending a vacation auto-reply. Every Gmail account has exactly one default send-as address, so the only legal value that clients may write to this field is `true`. Changing this from `false` to `true` for an address will result in this field becoming `false` for the other previous default address.",
                    "type": "boolean"
                },
                "treatAsAlias": {
                    "description": "Whether Gmail should treat this address as an alias for the user's primary email address. This setting only applies to custom \"from\" aliases.",
                    "type": "boolean"
                },
                "smtpMsa": {
                    "description": "An optional SMTP service that will be used as an outbound relay for mail sent using this alias. If this is empty, outbound mail will be sent directly from Gmail's servers to the destination SMTP service. This setting only applies to custom \"from\" aliases.",
                    "$ref": "SmtpMsa"
                },
                "verificationStatus": {
                    "description": "Indicates whether this address has been verified for use as a send-as alias. Read-only. This setting only applies to custom \"from\" aliases.",
                    "type": "string",
                    "enumDescriptions": [
                        "Unspecified verification status.",
                        "The address is ready to use as a send-as alias.",
                        "The address is awaiting verification by the owner."
                    ],
                    "enum": [
                        "verificationStatusUnspecified",
                        "accepted",
                        "pending"
                    ]
                }
            }
        },
        "SmtpMsa": {
            "id": "SmtpMsa",
            "description": "Configuration for communication with an SMTP service.",
            "type": "object",
            "properties": {
                "host": {
                    "description": "The hostname of the SMTP service. Required.",
                    "type": "string"
                },
                "port": {
                    "description": "The port of the SMTP service. Required.",
                    "type": "integer",
                    "format": "int32"
                },
                "username": {
                    "description": "The username that will be used for authentication with the SMTP service. This is a write-only field that can be specified in requests to create or update SendAs settings; it is never populated in responses.",
                    "type": "string"
                },
                "password": {
                    "description": "The password that will be used for authentication with the SMTP service. This is a write-only field that can be specified in requests to create or update SendAs settings; it is never populated in responses.",
                    "type": "string"
                },
                "securityMode": {
                    "description": "The protocol that will be used to secure communication with the SMTP service. Required.",
                    "type": "string",
                    "enumDescriptions": [
                        "Unspecified security mode.",
                        "Communication with the remote SMTP service is unsecured. Requires port 25.",
                        "Communication with the remote SMTP service is secured using SSL.",
                        "Communication with the remote SMTP service is secured using STARTTLS."
                    ],
                    "enum": [
                        "securityModeUnspecified",
                        "none",
                        "ssl",
                        "starttls"
                    ]
                }
            }
        },
        "ListSmimeInfoResponse": {
            "id": "ListSmimeInfoResponse",
            "type": "object",
            "properties": {
                "smimeInfo": {
                    "description": "List of SmimeInfo.",
                    "type": "array",
                    "items": {
                        "$ref": "SmimeInfo"
                    }
                }
            }
        },
        "SmimeInfo": {
            "id": "SmimeInfo",
            "description": "An S/MIME email config.",
            "type": "object",
            "properties": {
                "id": {
                    "description": "The immutable ID for the SmimeInfo.",
                    "type": "string"
                },
                "issuerCn": {
                    "description": "The S/MIME certificate issuer's common name.",
                    "type": "string"
                },
                "isDefault": {
                    "description": "Whether this SmimeInfo is the default one for this user's send-as address.",
                    "type": "boolean"
                },
                "expiration": {
                    "description": "When the certificate expires (in milliseconds since epoch).",
                    "type": "string",
                    "format": "int64"
                },
                "pem": {
                    "description": "PEM formatted X509 concatenated certificate string (standard base64 encoding). Format used for returning key, which includes public key as well as certificate chain (not private key).",
                    "type": "string"
                },
                "pkcs12": {
                    "description": "PKCS#12 format containing a single private/public key pair and certificate chain. This format is only accepted from client for creating a new SmimeInfo and is never returned, because the private key is not intended to be exported. PKCS#12 may be encrypted, in which case encryptedKeyPassword should be set appropriately.",
                    "type": "string",
                    "format": "byte"
                },
                "encryptedKeyPassword": {
                    "description": "Encrypted key password, when key is encrypted.",
                    "type": "string"
                }
            }
        },
        "CseIdentity": {
            "id": "CseIdentity",
            "description": "The client-side encryption (CSE) configuration for the email address of an authenticated user. Gmail uses CSE configurations to save drafts of client-side encrypted email messages, and to sign and send encrypted email messages.",
            "type": "object",
            "properties": {
                "emailAddress": {
                    "description": "The email address for the sending identity. The email address must be the primary email address of the authenticated user.",
                    "type": "string"
                },
                "primaryKeyPairId": {
                    "description": "If a key pair is associated, the ID of the key pair, CseKeyPair.",
                    "type": "string"
                },
                "signAndEncryptKeyPairs": {
                    "description": "The configuration of a CSE identity that uses different key pairs for signing and encryption.",
                    "$ref": "SignAndEncryptKeyPairs"
                }
            }
        },
        "SignAndEncryptKeyPairs": {
            "id": "SignAndEncryptKeyPairs",
            "description": "The configuration of a CSE identity that uses different key pairs for signing and encryption.",
            "type": "object",
            "properties": {
                "signingKeyPairId": {
                    "description": "The ID of the CseKeyPair that signs outgoing mail.",
                    "type": "string"
                },
                "encryptionKeyPairId": {
                    "description": "The ID of the CseKeyPair that encrypts signed outgoing mail.",
                    "type": "string"
                }
            }
        },
        "CseKeyPair": {
            "id": "CseKeyPair",
            "description": "A client-side encryption S/MIME key pair, which is comprised of a public key, its certificate chain, and metadata for its paired private key. Gmail uses the key pair to complete the following tasks: - Sign outgoing client-side encrypted messages. - Save and reopen drafts of client-side encrypted messages. - Save and reopen sent messages. - Decrypt incoming or archived S/MIME messages.",
            "type": "object",
            "properties": {
                "keyPairId": {
                    "description": "Output only. The immutable ID for the client-side encryption S/MIME key pair.",
                    "readOnly": true,
                    "type": "string"
                },
                "pkcs7": {
                    "description": "Input only. The public key and its certificate chain. The chain must be in [PKCS#7](https://en.wikipedia.org/wiki/PKCS_7) format and use PEM encoding and ASCII armor.",
                    "type": "string"
                },
                "pem": {
                    "description": "Output only. The public key and its certificate chain, in [PEM](https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail) format.",
                    "readOnly": true,
                    "type": "string"
                },
                "subjectEmailAddresses": {
                    "description": "Output only. The email address identities that are specified on the leaf certificate.",
                    "readOnly": true,
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                },
                "enablementState": {
                    "description": "Output only. The current state of the key pair.",
                    "readOnly": true,
                    "type": "string",
                    "enumDescriptions": [
                        "The current state of the key pair is not set. The key pair is neither turned on nor turned off.",
                        "The key pair is turned on. For any email messages that this key pair encrypts, Gmail decrypts the messages and signs any outgoing mail with the private key. To turn on a key pair, use the EnableCseKeyPair method.",
                        "The key pair is turned off. Authenticated users cannot decrypt email messages nor sign outgoing messages. If a key pair is turned off for more than 30 days, you can permanently delete it. To turn off a key pair, use the DisableCseKeyPair method."
                    ],
                    "enum": [
                        "stateUnspecified",
                        "enabled",
                        "disabled"
                    ]
                },
                "disableTime": {
                    "description": "Output only. If a key pair is set to `DISABLED`, the time that the key pair's state changed from `ENABLED` to `DISABLED`. This field is present only when the key pair is in state `DISABLED`.",
                    "readOnly": true,
                    "type": "string",
                    "format": "google-datetime"
                },
                "privateKeyMetadata": {
                    "description": "Metadata for instances of this key pair's private key.",
                    "type": "array",
                    "items": {
                        "$ref": "CsePrivateKeyMetadata"
                    }
                }
            }
        },
        "CsePrivateKeyMetadata": {
            "id": "CsePrivateKeyMetadata",
            "description": "Metadata for a private key instance.",
            "type": "object",
            "properties": {
                "privateKeyMetadataId": {
                    "description": "Output only. The immutable ID for the private key metadata instance.",
                    "readOnly": true,
                    "type": "string"
                },
                "kaclsKeyMetadata": {
                    "description": "Metadata for a private key instance managed by an external key access control list service.",
                    "$ref": "KaclsKeyMetadata"
                },
                "hardwareKeyMetadata": {
                    "description": "Metadata for hardware keys.",
                    "$ref": "HardwareKeyMetadata"
                }
            }
        },
        "KaclsKeyMetadata": {
            "id": "KaclsKeyMetadata",
            "description": "Metadata for private keys managed by an external key access control list service. For details about managing key access, see [Google Workspace CSE API Reference](https://developers.google.com/workspace/cse/reference).",
            "type": "object",
            "properties": {
                "kaclsUri": {
                    "description": "The URI of the key access control list service that manages the private key.",
                    "type": "string"
                },
                "kaclsData": {
                    "description": "Opaque data generated and used by the key access control list service. Maximum size: 8 KiB.",
                    "type": "string"
                }
            }
        },
        "HardwareKeyMetadata": {
            "id": "HardwareKeyMetadata",
            "description": "Metadata for hardware keys.",
            "type": "object",
            "properties": {
                "description": {
                    "description": "Description about the hardware key.",
                    "type": "string"
                }
            }
        },
        "DisableCseKeyPairRequest": {
            "id": "DisableCseKeyPairRequest",
            "description": "Requests to turn off a client-side encryption key pair.",
            "type": "object",
            "properties": {}
        },
        "EnableCseKeyPairRequest": {
            "id": "EnableCseKeyPairRequest",
            "description": "Requests to turn on a client-side encryption key pair.",
            "type": "object",
            "properties": {}
        },
        "ListCseIdentitiesResponse": {
            "id": "ListCseIdentitiesResponse",
            "type": "object",
            "properties": {
                "cseIdentities": {
                    "description": "One page of the list of CSE identities configured for the user.",
                    "type": "array",
                    "items": {
                        "$ref": "CseIdentity"
                    }
                },
                "nextPageToken": {
                    "description": "Pagination token to be passed to a subsequent ListCseIdentities call in order to retrieve the next page of identities. If this value is not returned or is the empty string, then no further pages remain.",
                    "type": "string"
                }
            }
        },
        "ListCseKeyPairsResponse": {
            "id": "ListCseKeyPairsResponse",
            "type": "object",
            "properties": {
                "cseKeyPairs": {
                    "description": "One page of the list of CSE key pairs installed for the user.",
                    "type": "array",
                    "items": {
                        "$ref": "CseKeyPair"
                    }
                },
                "nextPageToken": {
                    "description": "Pagination token to be passed to a subsequent ListCseKeyPairs call in order to retrieve the next page of key pairs. If this value is not returned, then no further pages remain.",
                    "type": "string"
                }
            }
        },
        "ObliterateCseKeyPairRequest": {
            "id": "ObliterateCseKeyPairRequest",
            "description": "Request to obliterate a CSE key pair.",
            "type": "object",
            "properties": {}
        },
        "ListFiltersResponse": {
            "id": "ListFiltersResponse",
            "description": "Response for the ListFilters method.",
            "type": "object",
            "properties": {
                "filter": {
                    "description": "List of a user's filters.",
                    "type": "array",
                    "items": {
                        "$ref": "Filter"
                    }
                }
            }
        },
        "Filter": {
            "id": "Filter",
            "description": "Resource definition for Gmail filters. Filters apply to specific messages instead of an entire email thread.",
            "type": "object",
            "properties": {
                "id": {
                    "description": "The server assigned ID of the filter.",
                    "type": "string"
                },
                "criteria": {
                    "description": "Matching criteria for the filter.",
                    "$ref": "FilterCriteria"
                },
                "action": {
                    "description": "Action that the filter performs.",
                    "$ref": "FilterAction"
                }
            }
        },
        "FilterCriteria": {
            "id": "FilterCriteria",
            "description": "Message matching criteria.",
            "type": "object",
            "properties": {
                "from": {
                    "description": "The sender's display name or email address.",
                    "type": "string"
                },
                "to": {
                    "description": "The recipient's display name or email address. Includes recipients in the \"to\", \"cc\", and \"bcc\" header fields. You can use simply the local part of the email address. For example, \"example\" and \"example@\" both match \"example@gmail.com\". This field is case-insensitive.",
                    "type": "string"
                },
                "subject": {
                    "description": "Case-insensitive phrase found in the message's subject. Trailing and leading whitespace are be trimmed and adjacent spaces are collapsed.",
                    "type": "string"
                },
                "query": {
                    "description": "Only return messages matching the specified query. Supports the same query format as the Gmail search box. For example, `\"from:someuser@example.com rfc822msgid: is:unread\"`.",
                    "type": "string"
                },
                "negatedQuery": {
                    "description": "Only return messages not matching the specified query. Supports the same query format as the Gmail search box. For example, `\"from:someuser@example.com rfc822msgid: is:unread\"`.",
                    "type": "string"
                },
                "hasAttachment": {
                    "description": "Whether the message has any attachment.",
                    "type": "boolean"
                },
                "excludeChats": {
                    "description": "Whether the response should exclude chats.",
                    "type": "boolean"
                },
                "size": {
                    "description": "The size of the entire RFC822 message in bytes, including all headers and attachments.",
                    "type": "integer",
                    "format": "int32"
                },
                "sizeComparison": {
                    "description": "How the message size in bytes should be in relation to the size field.",
                    "type": "string",
                    "enumDescriptions": [
                        "",
                        "Find messages smaller than the given size.",
                        "Find messages larger than the given size."
                    ],
                    "enum": [
                        "unspecified",
                        "smaller",
                        "larger"
                    ]
                }
            }
        },
        "FilterAction": {
            "id": "FilterAction",
            "description": "A set of actions to perform on a message.",
            "type": "object",
            "properties": {
                "addLabelIds": {
                    "description": "List of labels to add to the message.",
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                },
                "removeLabelIds": {
                    "description": "List of labels to remove from the message.",
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                },
                "forward": {
                    "description": "Email address that the message should be forwarded to.",
                    "type": "string"
                }
            }
        },
        "ImapSettings": {
            "id": "ImapSettings",
            "description": "IMAP settings for an account.",
            "type": "object",
            "properties": {
                "enabled": {
                    "description": "Whether IMAP is enabled for the account.",
                    "type": "boolean"
                },
                "autoExpunge": {
                    "description": "If this value is true, Gmail will immediately expunge a message when it is marked as deleted in IMAP. Otherwise, Gmail will wait for an update from the client before expunging messages marked as deleted.",
                    "type": "boolean"
                },
                "expungeBehavior": {
                    "description": "The action that will be executed on a message when it is marked as deleted and expunged from the last visible IMAP folder.",
                    "type": "string",
                    "enumDescriptions": [
                        "Unspecified behavior.",
                        "Archive messages marked as deleted.",
                        "Move messages marked as deleted to the trash.",
                        "Immediately and permanently delete messages marked as deleted. The expunged messages cannot be recovered."
                    ],
                    "enum": [
                        "expungeBehaviorUnspecified",
                        "archive",
                        "trash",
                        "deleteForever"
                    ]
                },
                "maxFolderSize": {
                    "description": "An optional limit on the number of messages that an IMAP folder may contain. Legal values are 0, 1000, 2000, 5000 or 10000. A value of zero is interpreted to mean that there is no limit.",
                    "type": "integer",
                    "format": "int32"
                }
            }
        },
        "PopSettings": {
            "id": "PopSettings",
            "description": "POP settings for an account.",
            "type": "object",
            "properties": {
                "accessWindow": {
                    "description": "The range of messages which are accessible via POP.",
                    "type": "string",
                    "enumDescriptions": [
                        "Unspecified range.",
                        "Indicates that no messages are accessible via POP.",
                        "Indicates that unfetched messages received after some past point in time are accessible via POP.",
                        "Indicates that all unfetched messages are accessible via POP."
                    ],
                    "enum": [
                        "accessWindowUnspecified",
                        "disabled",
                        "fromNowOn",
                        "allMail"
                    ]
                },
                "disposition": {
                    "description": "The action that will be executed on a message after it has been fetched via POP.",
                    "type": "string",
                    "enumDescriptions": [
                        "Unspecified disposition.",
                        "Leave the message in the `INBOX`.",
                        "Archive the message.",
                        "Move the message to the `TRASH`.",
                        "Leave the message in the `INBOX` and mark it as read."
                    ],
                    "enum": [
                        "dispositionUnspecified",
                        "leaveInInbox",
                        "archive",
                        "trash",
                        "markRead"
                    ]
                }
            }
        },
        "VacationSettings": {
            "id": "VacationSettings",
            "description": "Vacation auto-reply settings for an account. These settings correspond to the \"Vacation responder\" feature in the web interface.",
            "type": "object",
            "properties": {
                "enableAutoReply": {
                    "description": "Flag that controls whether Gmail automatically replies to messages.",
                    "type": "boolean"
                },
                "responseSubject": {
                    "description": "Optional text to prepend to the subject line in vacation responses. In order to enable auto-replies, either the response subject or the response body must be nonempty.",
                    "type": "string"
                },
                "responseBodyPlainText": {
                    "description": "Response body in plain text format. If both `response_body_plain_text` and `response_body_html` are specified, `response_body_html` will be used.",
                    "type": "string"
                },
                "responseBodyHtml": {
                    "description": "Response body in HTML format. Gmail will sanitize the HTML before storing it. If both `response_body_plain_text` and `response_body_html` are specified, `response_body_html` will be used.",
                    "type": "string"
                },
                "restrictToContacts": {
                    "description": "Flag that determines whether responses are sent to recipients who are not in the user's list of contacts.",
                    "type": "boolean"
                },
                "restrictToDomain": {
                    "description": "Flag that determines whether responses are sent to recipients who are outside of the user's domain. This feature is only available for Google Workspace users.",
                    "type": "boolean"
                },
                "startTime": {
                    "description": "An optional start time for sending auto-replies (epoch ms). When this is specified, Gmail will automatically reply only to messages that it receives after the start time. If both `startTime` and `endTime` are specified, `startTime` must precede `endTime`.",
                    "type": "string",
                    "format": "int64"
                },
                "endTime": {
                    "description": "An optional end time for sending auto-replies (epoch ms). When this is specified, Gmail will automatically reply only to messages that it receives before the end time. If both `startTime` and `endTime` are specified, `startTime` must precede `endTime`.",
                    "type": "string",
                    "format": "int64"
                }
            }
        },
        "LanguageSettings": {
            "id": "LanguageSettings",
            "description": "Language settings for an account. These settings correspond to the \"Language settings\" feature in the web interface.",
            "type": "object",
            "properties": {
                "displayLanguage": {
                    "description": "The language to display Gmail in, formatted as an RFC 3066 Language Tag (for example `en-GB`, `fr` or `ja` for British English, French, or Japanese respectively). The set of languages supported by Gmail evolves over time, so please refer to the \"Language\" dropdown in the Gmail settings for all available options, as described in the language settings help article. A table of sample values is also provided in the Managing Language Settings guide Not all Gmail clients can display the same set of languages. In the case that a user's display language is not available for use on a particular client, said client automatically chooses to display in the closest supported variant (or a reasonable default).",
                    "type": "string"
                }
            }
        },
        "ListForwardingAddressesResponse": {
            "id": "ListForwardingAddressesResponse",
            "description": "Response for the ListForwardingAddresses method.",
            "type": "object",
            "properties": {
                "forwardingAddresses": {
                    "description": "List of addresses that may be used for forwarding.",
                    "type": "array",
                    "items": {
                        "$ref": "ForwardingAddress"
                    }
                }
            }
        },
        "ForwardingAddress": {
            "id": "ForwardingAddress",
            "description": "Settings for a forwarding address.",
            "type": "object",
            "properties": {
                "forwardingEmail": {
                    "description": "An email address to which messages can be forwarded.",
                    "type": "string"
                },
                "verificationStatus": {
                    "description": "Indicates whether this address has been verified and is usable for forwarding. Read-only.",
                    "type": "string",
                    "enumDescriptions": [
                        "Unspecified verification status.",
                        "The address is ready to use for forwarding.",
                        "The address is awaiting verification by the owner."
                    ],
                    "enum": [
                        "verificationStatusUnspecified",
                        "accepted",
                        "pending"
                    ]
                }
            }
        },
        "AutoForwarding": {
            "id": "AutoForwarding",
            "description": "Auto-forwarding settings for an account.",
            "type": "object",
            "properties": {
                "enabled": {
                    "description": "Whether all incoming mail is automatically forwarded to another address.",
                    "type": "boolean"
                },
                "emailAddress": {
                    "description": "Email address to which all incoming messages are forwarded. This email address must be a verified member of the forwarding addresses.",
                    "type": "string"
                },
                "disposition": {
                    "description": "The state that a message should be left in after it has been forwarded.",
                    "type": "string",
                    "enumDescriptions": [
                        "Unspecified disposition.",
                        "Leave the message in the `INBOX`.",
                        "Archive the message.",
                        "Move the message to the `TRASH`.",
                        "Leave the message in the `INBOX` and mark it as read."
                    ],
                    "enum": [
                        "dispositionUnspecified",
                        "leaveInInbox",
                        "archive",
                        "trash",
                        "markRead"
                    ]
                }
            }
        },
        "ListDelegatesResponse": {
            "id": "ListDelegatesResponse",
            "description": "Response for the ListDelegates method.",
            "type": "object",
            "properties": {
                "delegates": {
                    "description": "List of the user's delegates (with any verification status). If an account doesn't have delegates, this field doesn't appear.",
                    "type": "array",
                    "items": {
                        "$ref": "Delegate"
                    }
                }
            }
        },
        "Delegate": {
            "id": "Delegate",
            "description": "Settings for a delegate. Delegates can read, send, and delete messages, as well as view and add contacts, for the delegator's account. See \"Set up mail delegation\" for more information about delegates.",
            "type": "object",
            "properties": {
                "delegateEmail": {
                    "description": "The email address of the delegate.",
                    "type": "string"
                },
                "verificationStatus": {
                    "description": "Indicates whether this address has been verified and can act as a delegate for the account. Read-only.",
                    "type": "string",
                    "enumDescriptions": [
                        "Unspecified verification status.",
                        "The address can act a delegate for the account.",
                        "A verification request was mailed to the address, and the owner has not yet accepted it.",
                        "A verification request was mailed to the address, and the owner rejected it.",
                        "A verification request was mailed to the address, and it expired without verification."
                    ],
                    "enum": [
                        "verificationStatusUnspecified",
                        "accepted",
                        "pending",
                        "rejected",
                        "expired"
                    ]
                }
            }
        }
    },
    "id": "gmail:v1",
    "batchPath": "batch",
    "title": "Gmail API",
    "basePath": "",
    "documentationLink": "https://developers.google.com/gmail/api/",
    "version": "v1",
    "ownerDomain": "google.com",
    "ownerName": "Google"
}