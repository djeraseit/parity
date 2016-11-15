// Copyright 2015, 2016 Ethcore (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

import React, { Component, PropTypes } from 'react';

import TxHash from '../../../ui/TxHash';
import VerificationStore from '../store';
const {
  POSTING_REQUEST, POSTED_REQUEST, REQUESTING_SMS
} = VerificationStore;

import styles from './sendRequest.css';

// TODO: move this to a better place
const nullable = (type) => PropTypes.oneOfType([ PropTypes.oneOf([ null ]), type ]);

export default class SendRequest extends Component {
  static propTypes = {
    step: PropTypes.any.isRequired,
    tx: nullable(PropTypes.any.isRequired)
  }

  render () {
    const { step, tx } = this.props;

    if (step === POSTING_REQUEST) {
      return (<p>A verification request will be sent to the contract. Please authorize this using the Parity Signer.</p>);
    }

    if (step === POSTED_REQUEST) {
      return (
        <div className={ styles.centered }>
          <TxHash hash={ tx } maxConfirmations={ 1 } />
          <p>Please keep this window open.</p>
        </div>
      );
    }

    if (step === REQUESTING_SMS) {
      return (<p>Requesting an SMS from the Parity server.</p>);
    }

    return null;
  }
}